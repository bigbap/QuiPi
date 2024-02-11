use quipi_sandbox::{
    QuiPiSandbox,
    WIDTH,
    HEIGHT
};

fn main() {
    let mut app = quipi_2d::QuiPi2D::init(
        QuiPiSandbox::new(),
        "Bouncing Shapes",
        WIDTH,
        HEIGHT,
    ).expect("There was a problem initializing QuiPi Sandbox");

    if let Err(e) = app.run() {
        eprintln!("QuiPi Sandbox ended unexpectedly: {}", e);
    };
}
