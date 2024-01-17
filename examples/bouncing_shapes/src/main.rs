use bouncing_shapes::MyGame;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let mut game = MyGame::new(
        WIDTH as f32,
        HEIGHT as f32
    ).expect("There was a problem initializing the game.");

    if let Err(e) = engine::engine::run(&mut game, "Bouncing Shapes", WIDTH, HEIGHT, true, false) {
        eprintln!("Game ended unexpectedly: {}", e);
    };
}
