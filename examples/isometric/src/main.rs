use ::skald::GFXFlags;
use isometric::{
    MyGame,
    WIDTH,
    HEIGHT
};

fn main() {
    let mut game = MyGame::new().expect("There was a problem initializing the game.");

    if let Err(e) = skald::run(
        &mut game,
        "Examples - Isometric",
        WIDTH,
        HEIGHT,
        vec![],
        vec![GFXFlags::DepthTest, GFXFlags::AlphaBlending]
    ) {
        eprintln!("Game ended unexpectedly: {}", e);
    };
}
