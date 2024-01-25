use bouncing_shapes::{
    MyGame,
    WIDTH,
    HEIGHT
};
use skald::GFXFlags;

fn main() {
    let mut game = MyGame::new().expect("There was a problem initializing the game.");

    if let Err(e) = skald::run(
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
