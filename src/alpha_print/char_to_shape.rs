use crate::geometry::Shape as sh;


pub fn convertor(x: isize, y: isize, sc: isize, c: char) -> (isize, std::vec::Vec<sh>) {
    return match c {
        'A' => (6 * sc, vec![
            sh::new_line(x, y + sc, x, y + 6 * sc),
            sh::new_line(x + 4 * sc, y + sc, x + 4 * sc, y + 6 * sc),
            sh::new_line(x + sc, y + 2 * sc, x + 3 * sc, y + 2 * sc),
            sh::new_line(x + sc, y, x + 3 * sc, y)
        ]),
        'B' => (6 * sc, vec![
            sh::new_line(x, y, x, y + 6 * sc),

            sh::new_line(x + 4 * sc, y + 3 * sc, x + 4 * sc, y + 5 * sc),
            sh::new_point(x + 4 * sc, y + sc),

            sh::new_line(x + sc, y, x + 3 * sc, y),
            sh::new_line(x + sc, y + 2 * sc, x + 3 * sc, y + 2 * sc),
            sh::new_line(x + sc, y + 6 * sc, x + 3 * sc, y + 6 * sc),

        ]),
        'O' => (6 * sc, vec![
            sh::new_line(x + sc, y, x + 3 * sc, y),
            sh::new_line(x + sc, y + 6 * sc, x + 3 * sc, y + 6 * sc),
            sh::new_line(x, y + sc, x, y + 5 * sc),
            sh::new_line(x + 4 * sc, y + sc, x + 4 * sc, y + 5 * sc),
        ]),
        _ => todo!("unknown character")
    };
}
