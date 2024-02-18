fn main() {
    if let Err(e) = bouncing_shapes::run() {
        eprintln!("Bouncing Shapes ended unexpectedly: {}", e);
    };
}
