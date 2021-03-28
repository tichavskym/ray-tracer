use env_logger::Env;
use ray_tracing::run;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    run();
}
