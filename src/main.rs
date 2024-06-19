use std::{collections::VecDeque, isize, ops::{Add, Sub}, thread::sleep, time::Duration, usize};

use minifb::{Key, Window, WindowOptions, Menu, MenuItem};
use rand::{Rng, SeedableRng};
fn n_mod_m <T: std::ops::Rem<Output = T> + std::ops::Add<Output = T> + Copy>
  (n: T, m: T) -> T {
    ((n % m) + m) % m
}

const WIDTH: usize = 24;
const HEIGHT: usize = 24;
const FPS: usize = 165;
const SPEED: usize = 2;

const _WHITE: u32 = 0xffffff;
const _GREY: u32 = 0x404040;
const _BLACK: u32 = 0x000000;
const _RED: u32 = 0x00ff0000;
const _GREEN: u32 = 0x0000ff00;
const _BLUE: u32 = 0x000000ff;
const _PURPLE: u32 = 0x800080;
const _YELLOW: u32 = 0xffff00;
const _ORANGE: u32 = 0x918c8a;

const BACK_COL: u32 = _WHITE;
const BODY_COL: u32 = _GREY;
const APPL_COL: u32 = _RED;
const WALL_COL: u32 = _BLACK;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const fn new(x: isize, y: isize) -> Point {
        return Point {x, y};
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    
}

impl Point {
    pub fn shift(self: &mut Self, dir: Direction) {
        match dir {
            Direction::Left     => self.x -= 1,
            Direction::Right    => self.x += 1,
            Direction::Up       => self.y -= 1,
            Direction::Down     => self.y += 1,
        };
    }

    pub fn shifted(self: &Self, dir: Direction) -> Point {
        return match dir {
            Direction::Left     => Point { x: self.x - 1, y: self.y},
            Direction::Right    => Point { x: self.x + 1, y: self.y},
            Direction::Up       => Point { x: self.x, y: self.y - 1},
            Direction::Down     => Point { x: self.x, y: self.y + 1},
        }
    }
}

#[derive(Clone, Debug)]
struct DrawBuffer {
    buf: Vec<u32>,
    bounds: Point,
}

impl DrawBuffer {
    pub fn new(x: isize, y: isize, col: u32) -> DrawBuffer {
        return DrawBuffer { buf: vec![col; (x * y) as usize], bounds: Point::new(x, y)};
    }

    pub fn get(self: &Self, x: isize, y: isize) -> u32 {
        return self.buf[(y * self.bounds.x + x) as usize];
    }

    pub fn set(self: &mut Self, x: isize, y: isize, col: u32) {
        self.buf[(y * self.bounds.x + x) as usize] = col;
    }

    pub fn as_vec_u32(self: &Self) -> &Vec<u32> {
        return &self.buf;
    }

    pub fn draw_line(self: &mut Self, st: Point, ls: Point) {
        if st.x != ls.x && st.y != ls.y {
            todo!();
        }
        if st.x != ls.x {
            let mut x = st.x;
            while x != ls.x {
                self.set(x, st.y, WALL_COL);
                x = n_mod_m(x + 1, self.bounds.x);
            }
            self.set(x, st.y, WALL_COL);
        } else {
            let mut y = st.y;
            while y != ls.y {
                self.set(st.x, y, WALL_COL);
                y = n_mod_m(y + 1, self.bounds.y);
            }
            self.set(st.x, y, WALL_COL);
        }
    }

    pub fn draw_rect(self: &mut Self, min: Point, max: Point, fil: bool) {
        if fil {
            todo!();
        }
        let mut x = min.y;
        while x != max.x {
            self.set(x, min.y, WALL_COL);
            self.set(x, max.y, WALL_COL);
            x = n_mod_m(x + 1, self.bounds.x);
        }
        self.set(x, min.y, WALL_COL);
        self.set(x, max.y, WALL_COL);

        let mut y = min.y;
        while y != max.y {
            self.set(min.x, y, WALL_COL);
            self.set(max.x, y, WALL_COL);
            y = n_mod_m(y + 1, self.bounds.y);
        }
        self.set(min.x, y, WALL_COL);
        self.set(max.x, y, WALL_COL);
    }

}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    pub fn oposite(self: &Self) -> Direction {
        return match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        };
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Obstacle {
    Point(Point),
    Line(Point, Point),
    Rectangle(Point, Point, bool),
}

impl Obstacle {
    pub fn new_point(x: isize, y: isize) -> Obstacle {
        return Obstacle::Point(Point::new(x, y));
    }

    pub fn new_line(stx: isize, sty: isize, lsx: isize, lsy: isize) -> Obstacle {
        return Obstacle::Line(Point::new(stx, sty), Point::new(lsx, lsy));
    }

    pub fn new_rect(minx: isize, miny: isize, maxx: isize, maxy: isize, filled: bool) -> Obstacle {
        return Obstacle::Rectangle(Point::new(minx, miny), Point::new(maxx, maxy), filled);
    }

    pub fn draw(self: &Self, buf: &mut DrawBuffer) {
        match self {
            Obstacle::Point(p) => buf.set(p.x, p.y, WALL_COL),
            Obstacle::Line(st, ls) => buf.draw_line(st.clone(), ls.clone()),
            Obstacle::Rectangle(st, ls, fil) => buf.draw_rect(st.clone(), ls.clone(), fil.clone())
        };
    }

}

#[derive(Clone, Debug)]
struct Snake {
    scales: std::collections::VecDeque<Point>,
    dir: Direction,
    buf: DrawBuffer,
    score: u32,
    rng: rand::prelude::StdRng,
    alive: bool
}

impl Snake {
    pub fn gen_pickups(self: &mut Self) {
        let mut x: isize = n_mod_m(self.rng.gen(), self.buf.bounds.x);
        let mut y: isize = n_mod_m(self.rng.gen(), self.buf.bounds.y);
        while self.buf.get(x, y) != BACK_COL {
            x = n_mod_m(self.rng.gen(), self.buf.bounds.x);
            y = n_mod_m(self.rng.gen(), self.buf.bounds.y);
        }
        self.buf.set(x, y, APPL_COL);
    }

    pub fn new(x: isize, y: isize, st_len: usize, maxx: isize, maxy: isize, dir: Direction, comp: bool, obs: &Vec<Obstacle>) -> Snake {
        let mut res = Snake { 
            scales: VecDeque::new(), 
            buf: DrawBuffer::new(maxx, maxy, BACK_COL), 
            dir, 
            score: 0,
            rng: if comp {
                rand::prelude::StdRng::seed_from_u64(0)
            } else {
                rand::prelude::StdRng::seed_from_u64(0)
            },
            alive: true,
        };
        let mut aux = Point::new(x, y);
        res.buf.set(aux.x, aux.y, res.head_color());
        res.scales.push_back(aux.clone());
        let op = dir.oposite();
        for _ in 1..st_len {
            aux.shift(op);
            aux.x = n_mod_m(aux.x, res.buf.bounds.x);
            aux.y = n_mod_m(aux.y, res.buf.bounds.y);
            res.buf.set(aux.x, aux.y, BODY_COL);
            res.scales.push_back(aux.clone());
        }
        for ob in obs.iter() {
            ob.draw(&mut res.buf);
        }
        assert!(res.scales.len() == st_len);
        res.gen_pickups();
        return res;
    }

    pub fn head_color(self: &Self) -> u32 {
        return match self.dir {
            Direction::Left => _BLUE,
            Direction::Right => _PURPLE,
            Direction::Up => _YELLOW,
            Direction::Down => _ORANGE,
        };
    }

    pub fn shift_draw(self: &mut Self) {
        let last_head = self.scales.front().unwrap().clone();
        let mut new_head = last_head.shifted(self.dir);
        new_head.x = n_mod_m(new_head.x, self.buf.bounds.x);
        new_head.y = n_mod_m(new_head.y, self.buf.bounds.y);
        let nxt_tile = self.buf.get(new_head.x, new_head.y);

        if nxt_tile == APPL_COL {
            self.score += 1;
            self.gen_pickups();
        } else if nxt_tile == BODY_COL || nxt_tile == WALL_COL {
            self.alive = false;
            return;
        } else {
            let cur_back = self.scales.back().unwrap();
            self.buf.set(cur_back.x, cur_back.y, BACK_COL);
            self.scales.pop_back();
        }
        if !self.scales.is_empty() {
            self.buf.set(last_head.x, last_head.y, BODY_COL);
        }
        self.buf.set(new_head.x, new_head.y, self.head_color());
        self.scales.push_front(new_head);
    }

    pub fn buf_as_vec_u32(self: &Self) -> &Vec<u32> {
        return self.buf.as_vec_u32();
    }

    pub fn parse_keys(self: &mut Self, keys: Vec<minifb::Key>) {
        for key in keys {
            let new_dir = match key {
                minifb::Key::Left | minifb::Key::A => Direction::Left,
                minifb::Key::Right | minifb::Key::D => Direction::Right,
                minifb::Key::Up | minifb::Key::W => Direction::Up,
                minifb::Key::Down | minifb::Key::S => Direction::Down,
                _ => self.dir,
            };
            if new_dir != self.dir.oposite() {
                self.dir = new_dir;
            }
        }
        let head = self.scales.front().unwrap().clone();
        self.buf.set(head.x, head.y, self.head_color());
    }
}

fn main() {
    let obs = vec![
        Obstacle::new_point(WIDTH as isize / 4, HEIGHT as isize / 4),
        Obstacle::new_point(WIDTH as isize / 4 * 3, HEIGHT as isize / 4),
        Obstacle::new_point(WIDTH as isize / 4, HEIGHT as isize / 4 * 3),
        Obstacle::new_point(WIDTH as isize / 4 * 3, HEIGHT as isize / 4 * 3),
        Obstacle::new_rect(0, 0, HEIGHT as isize - 1, WIDTH as isize - 1, false),
        /*
        Obstacle::new_line(2, HEIGHT as isize / 2, WIDTH as isize - 3, HEIGHT as isize / 2),
        Obstacle::new_line(WIDTH as isize / 2, 2, WIDTH as isize / 2, HEIGHT as isize - 3),

        Obstacle::new_line(WIDTH as isize / 2, 2, WIDTH as isize - 3, 2),
        Obstacle::new_line(2, HEIGHT as isize - 3, WIDTH as isize / 2, HEIGHT as isize - 3),

        Obstacle::new_line(2, 2, 2, HEIGHT as isize / 2),
        Obstacle::new_line(WIDTH as isize - 3, HEIGHT as isize / 2, WIDTH as isize - 3, HEIGHT as isize - 3),
        */

    ];
    let mut snake: Snake = Snake::new(
        10,
        3,
        5, 
        WIDTH as isize, 
        HEIGHT as isize, 
        Direction::Right,
        true,
        &obs
    );

    let mut opts = WindowOptions::default();
    opts.scale = minifb::Scale::FitScreen;
    opts.scale_mode = minifb::ScaleMode::AspectRatioStretch;
    opts.resize = true;
    opts.topmost = true;

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        opts,
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.set_background_color(0xff, 0xff, 0xff);

    let mut help = Menu::new("help").unwrap();
    let wall = MenuItem::new("black - wall", 0);
    help.add_menu_item(&wall);
    let body = MenuItem::new("grey - body", 1);
    help.add_menu_item(&body);
    let head = MenuItem::new("blue - head", 2);
    help.add_menu_item(&head);
    let apll = MenuItem::new("red - apple", 3);
    help.add_menu_item(&apll);
    window.add_menu(&help);

    window.set_target_fps(FPS);
    window
        .update_with_buffer(snake.buf_as_vec_u32(), WIDTH, HEIGHT)
        .unwrap();
    let mut cur_frame: usize = 0;
    while snake.alive && window.is_open() && !window.is_key_down(Key::Escape) {
        if cur_frame == FPS / SPEED {
            cur_frame = 0;
            snake.shift_draw();
        } else {
            cur_frame += 1;
        }
        snake.parse_keys(window.get_keys_pressed(minifb::KeyRepeat::Yes));
        window
            .update_with_buffer(snake.buf_as_vec_u32(), WIDTH, HEIGHT)
            .unwrap();

    }
    println!("final score: {}", snake.score);
}
