use std::time::Instant;

use image::{ImageBuffer, RgbaImage};

fn main() {
    let start = Instant::now();
    let mut c = Curve::new();
    for _ in 0 .. 8 {
        c = c.next();
    }
    println!("{:?}", start.elapsed());
    
    let segments = c.segments();
    println!("{:?}", segments);

    let bounds = c.bounds();
    let w = f64::ceil(bounds[1] - bounds[3]) as u32 + 1;
    let h = f64::ceil(bounds[0] - bounds[2]) as u32 + 1;
    println!("{} {}", w, h);

    let mut img: RgbaImage = ImageBuffer::new(w, h);
    let mut idx = 0.0;
    let inv_segments = 7.0*180.0 / (segments as f64);
    let start = Instant::now();
    for (x, y) in Points::from(&c.0) {
        let hue = (220.0 + idx*inv_segments) % 360.0;
        let t = hue / 60.0;
        let c = t - f64::floor(t);
        let i = 1.0 - c;
        let (r, g, b) = if t < 1.0 {
            (1.0, c, 0.0)
        } else if t < 2.0 {
            (i, 1.0, 0.0)
        } else if t < 3.0 {
            (0.0, 1.0, c)
        } else if t < 4.0 {
            (0.0, i, 1.0)
        } else if t < 5.0 {
            (c, 0.0, 1.0)
        } else {
            (1.0, 0.0, i)
        };
        let r = (r * 255.0) as u8;
        let g = (g * 255.0) as u8;
        let b = (b * 255.0) as u8;

        let x = f64::floor(x - bounds[3]) as u32;
        let y = f64::floor(y - bounds[2]) as u32;
        img.put_pixel(x, y, image::Rgba([r, g, b, 255]));
        idx += 1.0;
    }
    println!("{:?}", start.elapsed());
    img.save("gosper.png").unwrap();
}

#[derive(Copy, Clone, Debug)]
enum Node {
    A, B, L, R
}

macro_rules! pattern {
    ( $( $x:ident )* ) => {
        [ $( Node::$x, )* ]
    };
}

const NEXT_A: [Node; 15] = pattern!(A L B L L B R A R R A A R B L);
const NEXT_B: [Node; 15] = pattern!(R A L B B L L B L A R R A R B);

#[derive(Clone, Debug)]
struct Curve (Vec<Node>);

impl Curve {
    fn new() -> Self {
        Self(vec![Node::A])
    }

    fn next(self) -> Self {
        // On average, the list grows slightly more than 7x
        let mut next = Vec::with_capacity(8 * self.0.len());

        for node in self.0 {
            match node {
                Node::A => next.extend_from_slice(&NEXT_A),
                Node::B => next.extend_from_slice(&NEXT_B),
                _ => next.push(node),
            }
        }

        Self(next)
    }

    fn segments(&self) -> usize {
        let mut n = 0;
        for node in &self.0 {
            match node {
                Node::A => n += 1,
                Node::B => n += 1,
                _ => (),
            }
        }
        n
    }

    fn bounds(&self) -> [f64; 4] {
        let mut bounds = [0f64; 4];

        for (x, y) in Points::from(&self.0) {
            bounds[0] = f64::max(bounds[0], y);
            bounds[1] = f64::max(bounds[1], x);
            bounds[2] = f64::min(bounds[2], y);
            bounds[3] = f64::min(bounds[3], x);
        }

        bounds
    }
}

struct Points {
    idx: usize,
    nodes: Vec<Node>,
    dir: u8,
    x: f64,
    y: f64,
}

impl Points {
    fn from(nodes: &[Node]) -> Self {
        Self {
            idx: 0,
            nodes: nodes.to_vec(),
            dir: 0,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl Iterator for Points {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.idx >= self.nodes.len() {
                return None;
            }

            let node = self.nodes[self.idx];
            self.idx += 1;

            let mut nx = self.x;
            let mut ny = self.y;
            let s: f64 = f64::sqrt(0.75);
            match node {
                Node::L => {
                    self.dir = (self.dir + 1) % 6;
                    continue;
                },
                Node::R => {
                    self.dir = (self.dir + 5) % 6;
                    continue;
                },
                _ => {
                    match self.dir {
                        0 => {
                            nx += 1.0;
                        },
                        1 => {
                            nx += 0.5;
                            ny += s;
                        },
                        2 => {
                            nx -= 0.5;
                            ny += s;
                        },
                        3 => {
                            nx -= 1.0;
                        },
                        4 => {
                            nx -= 0.5;
                            ny -= s;
                        },
                        5 => {
                            nx += 0.5;
                            ny -= s;
                        },
                        _ => unreachable!("invalid direction"),
                    }
                },
            }

            self.x = nx;
            self.y = ny;

            return Some((nx, ny))
        }
    }
}
