use bouncing_shapes::{
    BouncingShapes,
    WIDTH,
    HEIGHT
};

fn main() {
    let mut app = quipi_2d::QuiPi2D::init(
        BouncingShapes::new(),
        "Bouncing Shapes",
        WIDTH,
        HEIGHT,
    ).expect("There was a problem initializing Bouncing Shapes");

    if let Err(e) = app.run() {
        eprintln!("Bouncing Shapes ended unexpectedly: {}", e);
    };
}
