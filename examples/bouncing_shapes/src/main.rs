use bouncing_shapes::{
    MyGame,
    WIDTH,
    HEIGHT
};
use engine::gfx::GFXFlags;

fn main() {
    let mut game = MyGame::new().expect("There was a problem initializing the game.");

    if let Err(e) = engine::engine::run(
        &mut game,
        "Bouncing Shapes",
        WIDTH,
        HEIGHT,
        vec![],
        vec![GFXFlags::AlphaBlending]
    ) {
        eprintln!("Game ended unexpectedly: {}", e);
    };
}
