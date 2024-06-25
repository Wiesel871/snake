use char_to_shape::convertor;

use crate::geometry::Shape as Sh;

mod char_to_shape;

pub fn convert(x: isize, y: isize, sc: isize, txt: &str) -> (isize, Vec<Sh>) {
    let mut res: Vec<Sh> = vec![];
    let mut shift: isize = 0;
    for c in txt.chars() {
        let (p, mut v) = convertor(x + shift, y, sc, c);
        let s = ron::to_string(&v).unwrap();
        println!("{} : {}", c, s);
        shift += p;
        res.append(&mut v);
    }
    return (shift, res);
}
