extern crate termion;

use termion::color::{Bg, Blue, Green, Red, Reset, White};
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

extern crate rand;
use rand::{Rng, SeedableRng, XorShiftRng};

use std::ops::{Index, IndexMut};
use std::collections::VecDeque;
use std::io::Write;

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

    fn print<W: Write>(&self, out: &mut W) {
        write!(out, "{}", termion::clear::All).unwrap();
        for i in 0..self.num_rows {
            write!(out, "{}", cursor::Goto(1, 1 + i as u16)).unwrap();
            for j in 0..self.num_cols {
                match *self.index((i, j)) {
                    b'r' => write!(out, "{}  ", Bg(Red)).unwrap(),
                    b'g' => write!(out, "{}  ", Bg(Green)).unwrap(),
                    b'b' => write!(out, "{}  ", Bg(Blue)).unwrap(),
                    b'w' => write!(out, "{}  ", Bg(White)).unwrap(),
                    other => panic!("illegal color {}", other),
                };
            }
            write!(out, "{}", Bg(Reset)).unwrap();
        }
        out.flush().unwrap();
    }

    fn expand(&mut self, target: u8) {
        let e0 = self.elements[0];
        if target == e0 {
            return;
        }

        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        while let Some((i, j)) = queue.pop_front() {
            if self[(i, j)] == e0 {
                self[(i, j)] = target;
                if i > 0 {
                    queue.push_back((i - 1, j));
                }
                if i + 1 < self.num_rows {
                    queue.push_back((i + 1, j));
                }
                if j > 0 {
                    queue.push_back((i, j - 1));
                }
                if j + 1 < self.num_cols {
                    queue.push_back((i, j + 1));
                }
            }
        }
    }

    fn is_uniform(&self) -> bool {
        let e0 = self.elements[0];
        self.elements.iter().all(|&e| e == e0)
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", cursor::Hide).unwrap();

    let mut prng = XorShiftRng::from_seed([42, 42, 42, 42]);
    let mut grid: Grid = Grid::random(&mut prng, 10, 10);
    grid[(0, 0)] = Grid::SEED;

    let mut history = Vec::new();

    grid.print(&mut stdout);
    for c in stdin.keys() {
        let evt = c.unwrap();
        match evt {
            Key::Char('q') => break,
            Key::Char('r') => {
                history.push('r');
                grid.expand(b'r');
            }
            Key::Char('g') => {
                history.push('g');
                grid.expand(b'g');
            }
            Key::Char('b') => {
                history.push('b');
                grid.expand(b'b');
            }
            _ => (),
        }
        grid.print(&mut stdout);
        if grid.is_uniform() {
            break;
        }
    }

    write!(
        stdout,
        "{}{}{}{:?}{}",
        termion::clear::All,
        cursor::Goto(1, 1),
        cursor::Show,
        history,
        cursor::Goto(1, 2)
    ).unwrap();
}
