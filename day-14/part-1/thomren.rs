use std::env::args;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

const SAND_SOURCE: (usize, usize) = (500, 0);
const MAX_WIDTH: usize = 1000;
const MAX_HEIGHT: usize = 200;

fn run(input: &str) -> isize {
    let mut cave = Cave::from_str(input).expect("Failed to parse cave");
    let mut count = 0;
    while cave.drop_sand().is_some() {
        count += 1;
    }
    count
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Air,
    Rock,
    Sand,
}

struct Cave {
    state: [[Cell; MAX_HEIGHT]; MAX_WIDTH],
    source: (usize, usize),
    ymax: usize,
}

impl Cave {
    fn drop_sand(&mut self) -> Option<(usize, usize)> {
        use Cell::*;
        let (mut x, mut y) = self.source;
        while y <= self.ymax {
            if let Air = self.state[x][y + 1] {
                y += 1;
            } else if let Air = self.state[x - 1][y + 1] {
                y += 1;
                x -= 1;
            } else if let Air = self.state[x + 1][y + 1] {
                y += 1;
                x += 1;
            } else {
                self.state[x][y] = Sand;
                return Some((x, y));
            }
        }
        None
    }
}

impl FromStr for Cave {
    type Err = Box<dyn std::error::Error>;

    #[allow(clippy::needless_range_loop)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut state = [[Cell::Air; MAX_HEIGHT]; MAX_WIDTH];
        let mut ymax = SAND_SOURCE.1;
        for line in s.lines() {
            let coordinates = line
                .split(" -> ")
                .map(|c| c.split_once(',').ok_or("Invalid coordinates").unwrap())
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .collect::<Vec<(usize, usize)>>();
            ymax = ymax.max(coordinates.iter().map(|&p| p.1).max().unwrap_or(0));

            for window in coordinates.windows(2) {
                let (from, to) = (window[0], window[1]);
                if from.0 == to.0 {
                    let x = from.0;
                    let (ymin, ymax) = (from.1.min(to.1), from.1.max(to.1));
                    for y in ymin..=ymax {
                        state[x][y] = Cell::Rock;
                    }
                } else if from.1 == to.1 {
                    let y = from.1;
                    let (xmin, xmax) = (from.0.min(to.0), from.0.max(to.0));
                    for x in xmin..=xmax {
                        state[x][y] = Cell::Rock;
                    }
                } else {
                    return Err(Box::from("Paths must be vertical or horizontal"));
                }
            }
        }

        Ok(Self {
            state,
            source: SAND_SOURCE,
            ymax,
        })
    }
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
