use bouncing_shapes::MyGame;

fn main() {
    let mut game = MyGame::new().expect("There was a problem initializing the game.");

    if let Err(e) = engine::engine::run(&mut game, "Bouncing Shapes", 800, 600, true, false) {
        eprintln!("Game ended unexpectedly: {}", e);
    };
}
