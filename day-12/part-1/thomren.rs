use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> usize {
    let height_map = HeightMap::new(input);
    let start = height_map.find(b'S').expect("Cannot find start");
    let end = height_map.find(b'E').expect("Cannot find end");
    let get_neighbors = Box::new(|pos| height_map.get_neighbors(pos));
    shortest_path(start, end, get_neighbors).expect("Cannot reach goal")
}

#[derive(Debug)]
struct HeightMap<'a> {
    heights: &'a [u8],
    width: usize,
    height: usize,
}

impl<'a> HeightMap<'a> {
    fn new(input: &'a str) -> Self {
        let heights = input.trim().as_bytes();
        let width = input.find('\n').unwrap_or(input.len());
        let height = (input.len() + 1) / (width + 1);
        HeightMap {
            heights,
            width,
            height,
        }
    }

    fn get(&self, pos: Position) -> usize {
        let x = self.heights[pos.0 * (self.width + 1) + pos.1];
        (match x {
            b'S' => b'a',
            b'E' => b'z',
            _ => x,
        }) as usize
    }

    fn find(&self, b: u8) -> Option<Position> {
        let x = self.heights.iter().position(|&x| x == b)?;
        Some(Position(x / (self.width + 1), x % (self.width + 1)))
    }

    fn get_neighbors(&self, pos: Position) -> Vec<State> {
        let mut res = Vec::new();
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (i, j) = (pos.0 as isize + dx, pos.1 as isize + dy);
            if i < 0 || j < 0 || i as usize >= self.height || j as usize >= self.width {
                continue;
            }

            let neighbor = Position(i as usize, j as usize);
            if self.get(neighbor) <= 1 + self.get(pos) {
                res.push(State {
                    cost: 1,
                    position: neighbor,
                });
            }
        }
        res
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    cost: usize,
    position: Position,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Position(usize, usize);

fn shortest_path<'a>(
    start: Position,
    end: Position,
    get_neighbors: Box<dyn Fn(Position) -> Vec<State> + 'a>,
) -> Option<usize> {
    let mut dist: HashMap<Position, usize> = HashMap::new();
    dist.insert(start, 0);
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(State {
        cost: 0,
        position: start,
    }));

    while let Some(Reverse(State { cost, position })) = heap.pop() {
        if position == end {
            return Some(cost);
        }

        if cost > dist.get(&position).cloned().unwrap_or_default() {
            continue;
        }

        for neighbor in get_neighbors(position) {
            let next = State {
                cost: cost + neighbor.cost,
                position: neighbor.position,
            };

            if next.cost < *dist.get(&next.position).unwrap_or(&std::usize::MAX) {
                heap.push(Reverse(next));
                dist.insert(next.position, next.cost);
            }
        }
    }

    None
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
            31
        )
    }
}
