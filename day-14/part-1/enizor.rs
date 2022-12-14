use aoc::enizor::bitset::{bitset_size, ArrayBitSet};
use std::env::args;
use std::ops::RangeInclusive;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

// Assume the cave fits in the square of size S centered on the initial position
const S: usize = 1024;
const N: usize = bitset_size(S * S);
const OFFSET: isize = S as isize / 2;
type Grid = ArrayBitSet<N>;

struct Cave {
    /// A bit is set iff the corresponding position is occupied
    grid: Grid,
    /// used either as the position of the marker for drawing lines
    /// or to track the position of a falling sand block
    pos: (usize, usize),
    /// lowest level of the cave
    lowest: usize,
    /// highest level of the cave: the sand freefalls until it reaches it
    highest: usize,
}

impl Cave {
    fn set_pos(&mut self, pos: (usize, usize)) {
        self.pos = pos;
        self.lowest = self.lowest.max(self.pos.1);
        self.highest = self.highest.min(self.pos.1);
    }

    fn draw(&mut self, end: (usize, usize)) {
        for j in Self::range(self.pos.1, end.1) {
            for i in Self::range(self.pos.0, end.0) {
                self.grid.set(j * S + i)
            }
        }
        self.lowest = self.lowest.max(end.1);
        self.highest = self.highest.min(end.1);
        self.pos = end;
    }

    fn range(x: usize, y: usize) -> RangeInclusive<usize> {
        x.min(y)..=x.max(y)
    }

    /// returns true if the sand block stopped
    fn drop_sand(&mut self) -> bool {
        self.pos = (500, 0);
        loop {
            if self.pos.1 == self.lowest {
                return false;
            }
            if !self.grid.test((self.pos.1 + 1) * S + self.pos.0) {
                self.pos.1 += 1;
            } else if !self.grid.test((self.pos.1 + 1) * S + self.pos.0 - 1) {
                self.pos.1 += 1;
                self.pos.0 -= 1;
            } else if !self.grid.test((self.pos.1 + 1) * S + self.pos.0 + 1) {
                self.pos.1 += 1;
                self.pos.0 += 1;
            } else {
                self.grid.set(self.pos.1 * S + self.pos.0);
                return true;
            }
        }
    }
}

fn run(input: &str) -> isize {
    let mut cave = Cave {
        grid: ArrayBitSet::new(),
        pos: (500, 0),
        lowest: 0,
        highest: usize::MAX,
    };
    for l in input.trim().lines() {
        let mut first = true;
        for point in l.split(" -> ") {
            let c = point.find(',').unwrap();
            let x = point[..c].parse().unwrap();
            let y = point[c + 1..].parse().unwrap();
            if first {
                cave.set_pos((x, y));
                first = false;
            } else {
                cave.draw((x, y));
            }
        }
    }
    let mut res = 0;
    while cave.drop_sand() {
        res += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"),
            24
        )
    }
}
