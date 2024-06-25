use std::path::PathBuf;

mod geometry;
mod game;


fn main() {
    game::game_loop(&PathBuf::new());
}
