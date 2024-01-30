use isometric::{
    MyGame,
    WIDTH,
    HEIGHT
};

fn main() {
    let mut game = MyGame::new().expect("There was a problem initializing the game.");

    if let Err(e) = quipi::run(
        &mut game,
        "Examples - Isometric",
        WIDTH,
        HEIGHT,
        vec![],
    ) {
        eprintln!("Game ended unexpectedly: {}", e);
    };
}
