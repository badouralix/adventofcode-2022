use std::collections::{HashSet, VecDeque};
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
    let end = height_map.find(b'E').expect("Cannot find end");
    let is_start = Box::new(|hm: &HeightMap, p: Position| hm.get(p) as u8 == b'a');
    shortest_path(&height_map, end, is_start).expect("Cannot reach goal")
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

    fn get_neighbors(&self, pos: Position) -> Vec<Position> {
        let x = pos.0 as isize;
        let y = pos.1 as isize;
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .into_iter()
            .filter(|&(i, j)| {
                i >= 0
                    && j >= 0
                    && (i as usize) < self.height
                    && (j as usize) < self.width
                    && self.get(Position(i as usize, j as usize)) + 1 >= self.get(pos)
            })
            .map(|(i, j)| Position(i as usize, j as usize))
            .collect()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Position(usize, usize);

#[allow(clippy::type_complexity)]
fn shortest_path(
    height_map: &HeightMap,
    start: Position,
    is_end: Box<dyn Fn(&HeightMap, Position) -> bool>,
) -> Option<usize> {
    let mut frontier = VecDeque::new();
    let mut seen = HashSet::new();
    frontier.push_back((0, start));

    while let Some((dist, position)) = frontier.pop_front() {
        if is_end(height_map, position) {
            return Some(dist);
        }

        if seen.contains(&position) {
            continue;
        }
        seen.insert(position);

        for neighbor in height_map.get_neighbors(position) {
            frontier.push_back((dist + 1, neighbor));
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
            29
        )
    }
}
