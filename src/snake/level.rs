use std::path::PathBuf;
use crate::geometry::{Point, Shape as Sh, Direction as Dir};
use serde::{Serialize, Deserialize};
use ron::from_str;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Level {
    pub height: isize,
    pub width: isize,
    pub snake_len: usize,
    pub snake_start: Point,
    pub start_dir: Dir,
    pub walls: Vec<Sh>,
}

impl Level {
    pub fn load(pth: &str) -> Level {
        let mut pbuf = PathBuf::new();        
        pbuf.push("levels");
        pbuf.push(pth);
        return from_str(std::fs::read_to_string(pbuf).unwrap().as_str()).unwrap();
    }
    
}
