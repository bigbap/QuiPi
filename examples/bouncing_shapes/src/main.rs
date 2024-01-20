use bouncing_shapes::{
    MyGame,
    WIDTH,
    HEIGHT
};

fn main() {
    let mut game = MyGame::new().expect("There was a problem initializing the game.");

    if let Err(e) = engine::engine::run(&mut game, "Bouncing Shapes", WIDTH, HEIGHT, true, false) {
        eprintln!("Game ended unexpectedly: {}", e);
    };
}
