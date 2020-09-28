use ray_tracing::run;

/// This is an implementation of the Ray Tracing In One Weekend book by Peter Shirley
/// If you want to generate an image, you should run `$ cargo run > image.ppm`
/// Progress info goes into standard error, standard output is the resulting .ppm file.
fn main() {
    run();
}