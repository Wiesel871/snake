use crate::geometry::Shape as Sh;

pub mod font;

pub fn convert(f: &font::Font, x: isize, y: isize, sc: isize, txt: &str) -> (isize, Vec<Sh>) {
    let mut res: Vec<Sh> = vec![];
    let mut shift: isize = 0;
    for c in txt.chars() {
        let (p, mut v) = f.convert(x + shift, y, sc, c);
        let s = ron::to_string(&v).unwrap();
        println!("{} : {}", c, s);
        shift += p;
        if c == 'f' || c == 'F' {
            shift -= 1;
        }
        res.append(&mut v);
    }
    return (shift, res);
}
