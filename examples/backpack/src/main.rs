use backpack::MyGame;

use backpack::WIDTH;
use backpack::HEIGHT;

fn main() {
    let mut my_game = MyGame::new().expect("there was a problem initializing the game");

    if let Err(e) = engine::engine::run(
        &mut my_game,
        "Game Engine - BackPack Example",
        WIDTH,
        HEIGHT,
        true,
        false
    ) {
        eprintln!("{e}")
    }
}
