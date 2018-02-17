extern crate termion;

use termion::color::{Bg, Blue, Color, Green, Red, Reset, White};

extern crate rand;
use rand::{Rng, SeedableRng, XorShiftRng};
use rand::distributions::{IndependentSample, Range};

use std::ops::{Index, IndexMut};

struct Grid {
    elements: Vec<u8>,
    num_rows: usize,
    num_cols: usize,
}

impl Index<(usize, usize)> for Grid {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &u8 {
        &self.elements[index.0 * self.num_cols + index.1]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut u8 {
        &mut self.elements[index.0 * self.num_cols + index.1]
    }
}

impl Grid {
    const SEED: u8 = b'w';
    const VALUES: [u8; 3] = [b'r', b'g', b'b'];
    pub fn random(prng: &mut rand::XorShiftRng, num_rows: usize, num_cols: usize) -> Grid {
        let mut elements = Vec::with_capacity(num_rows * num_cols);
        for _ in 0..num_rows * num_cols {
            elements.push(*prng.choose(&Grid::VALUES).unwrap());
        }
        Grid {
            elements,
            num_rows,
            num_cols,
        }
    }

    fn print(&self) {
        // print!("{}", termion::clear::All);
        // print!("{}", termion::cursor::Goto(1, 1));
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                match *self.index((i, j)) {
                    b'r' => print!("{}  ", Bg(Red)),
                    b'g' => print!("{}  ", Bg(Green)),
                    b'b' => print!("{}  ", Bg(Blue)),
                    b'w' => print!("{}  ", Bg(White)),
                    other => panic!("illegal color {}", other),
                };
            }
            println!("{}", Bg(Reset));
        }
    }
}

fn main() {
    let mut prng = XorShiftRng::from_seed([42, 42, 42, 42]);
    let mut grid: Grid = Grid::random(&mut prng, 10, 10);
    grid[(0, 0)] = Grid::SEED;
    grid.print();
}
