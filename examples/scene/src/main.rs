use scene::MyGame;

use scene::WIDTH;
use scene::HEIGHT;

fn main() {
    let mut my_game = MyGame::new(WIDTH, HEIGHT).expect("there was a problem initializing the game");

    if let Err(e) = engine::engine::run(
        &mut my_game,
        "Game Engine - Scene Example",
        WIDTH,
        HEIGHT,
        false,
        true
    ) {
        eprintln!("{e}")
    }
}
