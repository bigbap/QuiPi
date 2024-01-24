use isometric::{
    MyGame,
    WIDTH,
    HEIGHT
};

use engine::engine;

fn main() {
    let mut game = MyGame::new().expect("There was a problem initializing the game.");

    if let Err(e) = engine::run(
        &mut game,
        "Examples - Isometric",
        WIDTH,
        HEIGHT,
        vec![
            engine::Flags::DepthTest
        ]
    ) {
        eprintln!("Game ended unexpectedly: {}", e);
    };
}
