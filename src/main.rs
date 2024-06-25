use std::path::PathBuf;

mod geometry;
mod game;
mod alpha_print;


fn main() {
    game::game_loop(&PathBuf::new());
}
