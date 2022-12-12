use std::collections::VecDeque;
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

const MIN_ALTITUDE: u8 = b'a' - 2;
const VISITED_BIT: u8 = 0x80;
const VALUE_MASK: u8 = !VISITED_BIT;

#[derive(Debug)]
struct Map {
    width: usize,
    elevation: Vec<u8>,
    end: usize,
}

impl Map {
    fn neighbors(&self, pos: usize) -> [usize; 4] {
        [pos - self.width, pos - 1, pos + 1, pos + self.width]
    }

    fn from_str(input: &str) -> Self {
        let width = input.find('\n').unwrap() + 2;
        let length = (input.len() + 1) / (width - 1) + 2;
        let mut elevation = Vec::with_capacity(width * length);
        elevation.resize(width + 1, MIN_ALTITUDE);
        let mut end = 0;
        for &b in input.as_bytes().iter() {
            if b == b'\n' {
                elevation.push(MIN_ALTITUDE);
                elevation.push(MIN_ALTITUDE);
            } else if b == b'S' {
                elevation.push(b'a');
            } else if b == b'E' {
                end = elevation.len();
                elevation.push(b'z');
            } else {
                elevation.push(b);
            }
        }
        elevation.resize(width * length, MIN_ALTITUDE);
        Self {
            width,
            elevation,
            end,
        }
    }

    fn solve(&mut self) -> i32 {
        let mut stack = VecDeque::with_capacity(self.width * 4);
        stack.push_back((self.end, 0));
        while let Some((pos, distance)) = stack.pop_front() {
            for n in self.neighbors(pos) {
                if (self.elevation[n] & VALUE_MASK) + 1 >= (self.elevation[pos] & VALUE_MASK)
                    && (self.elevation[n] & VISITED_BIT == 0)
                {
                    if self.elevation[n] == b'a' {
                        return distance + 1;
                    }
                    self.elevation[n] |= VISITED_BIT;
                    stack.push_back((n, distance + 1));
                }
            }
        }
        -1
    }
}

fn run(input: &str) -> i32 {
    let mut map = Map::from_str(input.trim());
    map.solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"),
            29
        )
    }
}
