use aoc::enizor::bitset::*;
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

// Assume the cave fits in the square spanning [0, 1024) x [0, 256)
const W: usize = 1024;
const W_0: isize = 0;
const L: usize = 256;
const L_O: isize = 0;
const N: usize = bitset_size(W * L);
type Grid = GridBitSet<N, W, W_0, L, L_O>;

struct Cave {
    /// A bit is set iff the corresponding position is occupied
    grid: Grid,
    // A bit is set iff sand can fall into the corresponding position
    accessible: Grid,
    /// used either as the position of the marker for drawing lines
    /// or to track the position of a falling sand block
    pos: (isize, isize),
    /// lowest level of the cave
    lowest: isize,
}

impl Cave {
    fn set_pos(&mut self, pos: (isize, isize)) {
        self.pos = pos;
        self.lowest = self.lowest.max(self.pos.1);
    }

    fn draw(&mut self, end: (isize, isize)) {
        for j in Self::range(self.pos.1, end.1) {
            for i in Self::range(self.pos.0, end.0) {
                self.grid.set((i, j))
            }
        }
        self.lowest = self.lowest.max(end.1);
        self.pos = end;
    }

    fn range(x: isize, y: isize) -> RangeInclusive<isize> {
        x.min(y)..=x.max(y)
    }

    fn try_place_sand(&mut self, x: isize, y: isize) {
        if !self.grid.test((x, y))
            && (self.accessible.test((x, y - 1))
                || self.accessible.test((x - 1, y - 1))
                || self.accessible.test((x + 1, y - 1)))
        {
            self.accessible.set((x, y));
        }
    }

    fn place_all_sand(&mut self) -> u32 {
        self.accessible.set((500, 0));
        let mut y = 0;
        let mut x0 = 500;
        let mut x1 = 500;
        while y <= self.lowest {
            y += 1;
            x0 -= 1;
            x1 += 1;
            for x in x0..=x1 {
                self.try_place_sand(x, y)
            }
        }
        self.accessible.bitset.count_ones()
    }
}

fn run(input: &str) -> u32 {
    let mut cave = Cave {
        grid: Grid::new(),
        accessible: Grid::new(),
        pos: (500, 0),
        lowest: 0,
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
    cave.place_all_sand()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"),
            93
        )
    }
}
