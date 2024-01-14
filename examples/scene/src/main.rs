use scene::MyGame;

fn main() {
    let mut my_game = MyGame::new().expect("there was a problem initializing the game");

    if let Err(e) = engine::engine::run(
        &mut my_game,
        "Game Engine - Scene Example",
        1600,
        900,
        false,
        true
    ) {
        eprintln!("{e}")
    }
}
