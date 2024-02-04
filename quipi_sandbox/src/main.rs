use quipi_sandbox::{
    QuiPiSandbox,
    WIDTH,
    HEIGHT
};

fn main() {
    let mut sandbox = QuiPiSandbox::new().expect("There was a problem initializing QuiPi Sandbox");

    if let Err(e) = quipi::run(
        &mut sandbox,
        "Bouncing Shapes",
        WIDTH,
        HEIGHT,
    ) {
        eprintln!("QuiPi Sandbox ended unexpectedly: {}", e);
    };
}
