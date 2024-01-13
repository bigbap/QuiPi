use triangle::MyGame;

pub static ASSET_PATH: &str = "./assets/";

fn main() {
    let mut my_game = MyGame::new().expect("there was a problem initializing the game");

    if let Err(e) = engine::engine::run(
        &mut my_game,
        "Game Engine - Triangle Example",
        800,
        600,
        true,
        false
    ) {
        eprintln!("{e}")
    }
}
