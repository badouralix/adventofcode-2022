use aoc::enizor::bitset::*;
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

// Assume the rope stays in the square of size S centered on the initial position

const S: usize = 1024;
const W: usize = S;
const W_0: isize = -(S as isize) / 2;
const L: usize = S;
const L_O: isize = -(S as isize) / 2;
const N: usize = bitset_size(W * L);
type Grid = GridBitSet<N, W, W_0, L, L_O>;

#[derive()]
struct Rope {
    links: [(isize, isize); 10],
    grid: Grid,
}

impl Rope {
    fn new() -> Self {
        Self {
            links: [(0, 0); 10],
            grid: Grid::new(),
        }
    }

    fn step(&mut self, direction: (isize, isize)) {
        self.links[0].0 += direction.0;
        self.links[0].1 += direction.1;
        for i in 1..10 {
            let d = match (
                self.links[i - 1].0 - self.links[i].0,
                self.links[i - 1].1 - self.links[i].1,
            ) {
                (x, y) if x.abs() >= 2 || y.abs() >= 2 => (x.signum(), y.signum()),
                _ => (0, 0),
            };
            self.links[i].0 += d.0;
            self.links[i].1 += d.1;
        }
        self.grid.set(self.links[9]);
    }

    fn res(&self) -> u32 {
        self.grid.bitset.count_ones()
    }
}

fn run(input: &str) -> u32 {
    let mut rope = Rope::new();
    for line in input.lines() {
        let mut word = line.split_ascii_whitespace();
        let d = match word.next().unwrap() {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!("Unknown direction"),
        };
        let length = word
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("Failed to parse input");
        for _ in 0..length {
            rope.step(d);
        }
    }
    rope.res()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"),
            1
        );
        assert_eq!(
            run("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"),
            36
        );
    }
}
