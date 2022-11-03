use logging::Frog;

fn main() {
    // Initialize env_logger using the init() function at the top level of the library
    let _ = env_logger::try_init();

    // Run this program with `cargo run` and take a look at the default output.
    // - run it again with an explicit log level, like `RUST_LOG=info cargo run`
    // - or `RUST_LOG=trace cargo run`
    let mut skippy = Frog::new();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.sleep();
    skippy.sleep();

}
