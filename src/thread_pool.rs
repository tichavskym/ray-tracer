/// This file is taken from (based on) Rust book. Its repository can be found here:
/// https://github.com/rust-lang/book

use std::error::Error;
use std::fmt::Formatter;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;
use std::{fmt, thread};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

/// `Worker` is an abstraction of a thread.
struct Worker {
    thread: Option<JoinHandle<()>>,
    id: u8,
}

impl Worker {
    /// Creates a new thread that waits for jobs wrapped in `Message` and then
    /// executes them. It stops when it gets `Message::Terminate`. All messages
    /// are transferred through the channel.
    fn new(id: u8, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        // The closure *can outlive* the function `new`, so it has to take
        // ownership of `receiver`.
        let thread = thread::spawn(move || {
            log::info!("Thread spawned");
            loop {
                // The lock is not assigned to a variable and therefore is released
                // as soon as the `let job` statement ends (the lock is held during
                // the call to recv)
                let message = receiver
                    .lock()
                    .expect(
                        "Mutex is probably in a poisoned state (some thread 
                        panicked while holding the lock) and therefore this thread
                        cannot get access to the Mutex.",
                    )
                    .recv()
                    .expect("The sending side of the channel has probably shut down.");

                match message {
                    Message::NewJob(job) => {
                        log::debug!("Worker {} got a job; executing.", id);
                        job(); // calling closure
                    }
                    Message::Terminate => {
                        log::debug!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// * `size` is the number of threads in the pool. If the size is 0,
    /// function returns a custom error `PoolCreationError`.
    pub fn new(size: u8) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            log::info!("Started creating a thread pool");
            let (sender, receiver) = mpsc::channel::<Message>();
            let receiver = Arc::new(Mutex::new(receiver));

            // More efficient than Vec::new()
            let mut workers = Vec::with_capacity(size as usize);
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }

            Ok(ThreadPool { workers, sender })
        } else {
            Err(PoolCreationError)
        }
    }

    /// Thread pool executes the closure.
    /// * `Send` to transfer the closure from one thread to another.
    /// * `'static` because we don’t know how long will it take to execute it.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(f);
        self.sender.send(Message::NewJob(job)).expect(
            "Receiving ends stopped receiving. I don't expect to see 
                    this error. Ever.",
        );
    }
}

/// Implementing drop trait so that threads finish their jobs before closing.
impl Drop for ThreadPool {
    fn drop(&mut self) {
        log::info!("Sending terminate message to all workers");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            log::info!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[derive(Debug)]
pub struct PoolCreationError;
impl Error for PoolCreationError {}
impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot create thread pool containing zero threads.")
    }
}
