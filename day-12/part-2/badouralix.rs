use std::collections::VecDeque;
use std::env::args;
use std::time::Instant;

#[derive(Debug)]
struct Map {
    rows: Vec<Vec<u8>>,
    height: usize,
    width: usize,
    end: (usize, usize),
}

impl Map {
    fn new_from_input(input: &str) -> Map {
        let rows = input
            .lines()
            .map(|line| {
                line.replace('S', "a")
                    .replace('E', "z")
                    .as_bytes()
                    .to_owned()
            })
            .collect();
        let height = input.matches('\n').count() + 1;
        let width = input.chars().position(|c| c == '\n').unwrap();
        let end = (
            input.chars().position(|c| c == 'E').unwrap() / (width + 1),
            input.chars().position(|c| c == 'E').unwrap() % (width + 1),
        );

        Map {
            rows,
            height,
            width,
            end,
        }
    }

    fn find_all_start_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for i in 0..self.height {
            for j in 0..self.width {
                if self.rows[i][j] == b'a' {
                    positions.push((i, j));
                }
            }
        }
        positions
    }

    fn find_shortest_path(&self, start: (usize, usize)) -> Option<usize> {
        let mut distances = vec![vec![None; self.width]; self.height];
        let mut queue = VecDeque::<((usize, usize), usize)>::new();

        queue.push_back((start, 0));
        while !queue.is_empty() {
            let (position, distance) = queue.pop_front().unwrap();
            match distances[position.0][position.1] {
                Some(v) if v <= distance => {}
                _ => {
                    distances[position.0][position.1] = Some(distance);
                    if position.0 > 0
                        && self.rows[position.0 - 1][position.1]
                            <= self.rows[position.0][position.1] + 1
                    {
                        queue.push_back(((position.0 - 1, position.1), distance + 1));
                    }
                    if position.0 < self.height - 1
                        && self.rows[position.0 + 1][position.1]
                            <= self.rows[position.0][position.1] + 1
                    {
                        queue.push_back(((position.0 + 1, position.1), distance + 1));
                    }
                    if position.1 > 0
                        && self.rows[position.0][position.1 - 1]
                            <= self.rows[position.0][position.1] + 1
                    {
                        queue.push_back(((position.0, position.1 - 1), distance + 1));
                    }
                    if position.1 < self.width - 1
                        && self.rows[position.0][position.1 + 1]
                            <= self.rows[position.0][position.1] + 1
                    {
                        queue.push_back(((position.0, position.1 + 1), distance + 1));
                    }
                }
            }
        }

        distances[self.end.0][self.end.1]
    }
}

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> usize {
    let map = Map::new_from_input(input);
    let mut result = usize::MAX;

    for start in map.find_all_start_positions() {
        if let Some(v) = map.find_shortest_path(start) {
            result = usize::min(result, v);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
                .trim()),
            29
        )
    }
}
