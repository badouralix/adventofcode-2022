use std::collections::VecDeque;
use std::env::args;
use std::time::Instant;

#[derive(Debug)]
struct Map {
    rows: Vec<Vec<u8>>,
    height: usize,
    width: usize,
    start: (usize, usize),
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
        let start = (
            input.chars().position(|c| c == 'S').unwrap() / (width + 1),
            input.chars().position(|c| c == 'S').unwrap() % (width + 1),
        );
        let end = (
            input.chars().position(|c| c == 'E').unwrap() / (width + 1),
            input.chars().position(|c| c == 'E').unwrap() % (width + 1),
        );

        Map {
            rows,
            height,
            width,
            start,
            end,
        }
    }

    fn find_shortest_path(&self) -> usize {
        let mut distances = vec![vec![None; self.width]; self.height];
        let mut queue = VecDeque::<((usize, usize), usize)>::new();

        queue.push_back((self.start, 0));
        while !queue.is_empty() {
            let (position, distance) = queue.pop_front().unwrap();
            match distances[position.0][position.1] {
                Some(v) if v <= distance => {}
                _ => {
                    if self.rows[position.0][position.1] == b'a' {
                        distances[position.0][position.1] = Some(0)
                    } else {
                        distances[position.0][position.1] = Some(distance);
                    }

                    if position.0 > 0
                        && self.rows[position.0 - 1][position.1]
                            <= self.rows[position.0][position.1] + 1
                    {
                        queue.push_back((
                            (position.0 - 1, position.1),
                            distances[position.0][position.1].unwrap() + 1,
                        ));
                    }
                    if position.0 < self.height - 1
                        && self.rows[position.0 + 1][position.1]
                            <= self.rows[position.0][position.1] + 1
                    {
                        queue.push_back((
                            (position.0 + 1, position.1),
                            distances[position.0][position.1].unwrap() + 1,
                        ));
                    }
                    if position.1 > 0
                        && self.rows[position.0][position.1 - 1]
                            <= self.rows[position.0][position.1] + 1
                    {
                        queue.push_back((
                            (position.0, position.1 - 1),
                            distances[position.0][position.1].unwrap() + 1,
                        ));
                    }
                    if position.1 < self.width - 1
                        && self.rows[position.0][position.1 + 1]
                            <= self.rows[position.0][position.1] + 1
                    {
                        queue.push_back((
                            (position.0, position.1 + 1),
                            distances[position.0][position.1].unwrap() + 1,
                        ));
                    }
                }
            }
        }

        distances[self.end.0][self.end.1].unwrap()
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
    map.find_shortest_path()
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
