use skald::Flags;
use scene::MyGame;

use scene::WIDTH;
use scene::HEIGHT;

fn main() {
    let mut my_game = MyGame::new().expect("there was a problem initializing the game");

    if let Err(e) = skald::run(
        &mut my_game,
        "Game Engine - Scene Example",
        WIDTH,
        HEIGHT,
        vec![Flags::HideMouse, Flags::RelativeMouseMode],
    ) {
        eprintln!("{e}")
    }
}
