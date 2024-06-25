use std::{collections::HashMap, path::PathBuf};
use std::cmp::max;

use crate::geometry::Shape as Sh;


pub struct Font {
    map: HashMap<char, Vec<Sh>>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum KV {
    #[serde(untagged)]
    KV(char, Vec<Sh>)
}

impl Font {
    pub fn load(pth: &str) -> Font {
        let mut pbuf = PathBuf::new();        
        pbuf.push("font");
        pbuf.push(pth);
        let mut res = Font { map: HashMap::new() };
        for line in std::fs::read_to_string(pbuf).unwrap().lines() {
            let (c, v) = ron::from_str(line).unwrap();
            res.map.insert(c, v);
        }
        return res;
    }

    pub fn convert(&self, x: isize, y: isize, sc: isize, c: char) -> (isize, Vec<Sh>) {
        let mut res = Vec::new();
        let mut maxx = 0;
        for s in self.map[&c].iter() {
            res.push(match s {
                Sh::Point(p) => { 
                    maxx = max(maxx, p.x); 
                    Sh::new_point(x + p.x * sc, y + p.y * sc)
                },
                Sh::Line(mn, mx) => {
                    maxx = max(maxx, mx.x); 
                    Sh::new_line(x + mn.x * sc, y + mn.y * sc, x + mx.x * sc, y + mx.y * sc)
                },
                Sh::Rectangle(min, mx, fil) => {
                    maxx = max(maxx, mx.x); 
                    Sh::new_rect(x + min.x * sc, y + min.y * sc, x + mx.x * sc, y + mx.y * sc, *fil)
                },
            });
        }
        return (maxx + 2, res);
    }
}
