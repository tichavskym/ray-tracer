# Ray tracer 

I wrote this program in order to learn Rust and computer graphics. The logic is
taken from an awesome book [Ray Tracing In One
Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by
Peter Shirley.

This program supports multi-threading using modified thread pool from [Rust
book](https://doc.rust-lang.org/stable/book/ch20-02-multithreaded.html).

## Usage

*It is still WIP and not user-friendly yet.*

The config is done through constants in `src/lib.rs` file and scene setup in
`set_scene_objects` function in the same file.

Compilation and execution should be done with `cargo run --release` for
performance reasons.
