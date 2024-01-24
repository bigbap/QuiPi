use engine::engine::Flags;
use engine::gfx::GFXFlags;
use scene::MyGame;

use scene::WIDTH;
use scene::HEIGHT;

fn main() {
    let mut my_game = MyGame::new().expect("there was a problem initializing the game");

    if let Err(e) = engine::engine::run(
        &mut my_game,
        "Game Engine - Scene Example",
        WIDTH,
        HEIGHT,
        vec![Flags::HideMouse, Flags::RelativeMouseMode],
        vec![GFXFlags::DepthTest]
    ) {
        eprintln!("{e}")
    }
}
