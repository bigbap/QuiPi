use triangle::MyGame;

pub static ASSET_PATH: &str = "./assets/";

fn main() {
    let mut my_game = MyGame::new().expect("there was a problem initializing the game");

    if let Err(e) = engine::engine::run(&mut my_game, "Learn OpenGL", 800, 600) {
        eprintln!("{e}")
    }
}
