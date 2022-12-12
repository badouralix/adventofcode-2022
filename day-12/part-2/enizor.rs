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

const MIN_ALTITUDE: u8 = 1;

#[derive(Debug)]
struct Map {
    width: usize,
    length: usize,
    elevation: Vec<u8>,
    access_time: Vec<i32>,
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
        let mut access_time = vec![-1; width * length];
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
                access_time[end] = 0;
                elevation.push(b'z');
            } else {
                elevation.push(b);
            }
        }
        elevation.resize(width * length, MIN_ALTITUDE);
        Self {
            width,
            length,
            elevation,
            access_time,
            end,
        }
    }

    fn solve(&mut self) -> i32 {
        let mut stack = VecDeque::with_capacity((self.width - 2) * (self.length - 2));
        stack.push_back(self.end);
        while let Some(pos) = stack.pop_front() {
            for n in self.neighbors(pos) {
                if self.elevation[n] + 1 >= self.elevation[pos] {
                    let new_route = self.access_time[pos] + 1;
                    if self.access_time[n] < 0 || new_route < self.access_time[n] {
                        if self.elevation[n] == b'a' {
                            return new_route;
                        }
                        self.access_time[n] = new_route;
                        stack.push_back(n);
                    }
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
