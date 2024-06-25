use std::path::PathBuf;

mod geometry;
mod game;
mod alpha_print;


fn main() {
    let kv = alpha_print::font::KV::KV('A', vec![geometry::Shape::new_point(0, 0)]);
    let s = ron::to_string(&kv).unwrap();
    println!("A: {}", s);
    game::game_loop(&PathBuf::new());
}
