use bouncing_shapes::{
    MyGame,
    WIDTH,
    HEIGHT
};

fn main() {
    let mut game = MyGame::new().expect("There was a problem initializing the game.");

    if let Err(e) = quipi::run(
        &mut game,
        "Bouncing Shapes",
        WIDTH,
        HEIGHT,
        vec![],
    ) {
        eprintln!("Game ended unexpectedly: {}", e);
    };
}
