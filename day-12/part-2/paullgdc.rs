use std::cmp::Reverse;
use std::collections::VecDeque;
use std::env::args;
use std::time::Instant;

use aoc::paullgdc::matrix::Matrix;
use aoc::paullgdc::tokenize::Tokenizer;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

#[derive(Debug)]
struct Map {
    heights: Matrix<u8>,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_heights(tokenizer: &mut Tokenizer) -> Option<Map> {
    let mut heights = Vec::with_capacity(tokenizer.left().len());
    let mut start = None;
    let mut end = None;
    let mut j = 0;
    let mut i = 0;
    while !tokenizer.end() {
        i = 0;
        while !tokenizer.end() && tokenizer.next_nth_byte(0) != Some(b'\n') {
            heights.push(match tokenizer.next_nth_byte(0)? {
                b'S' => {
                    start = Some((i, j));
                    b'a'
                }
                b'E' => {
                    end = Some((i, j));
                    b'z'
                }
                c => c,
            });
            tokenizer.advance(1);
            i += 1;
        }
        tokenizer.eat_byte_or_end(b'\n')?;
        j += 1;
    }
    Some(Map {
        heights: Matrix::from_vec(heights, i)?,
        start: start?,
        end: end?,
    })
}

fn find_shortest_path(map: &Map) -> Option<u32> {
    let mut queue = VecDeque::with_capacity(map.heights.dims.0 * map.heights.dims.1);
    let mut visited = map.heights.map(|_| false);
    for j in 0..map.heights.dims.1 {
        for i in 0..map.heights.dims.0 {
            if map.heights[(i, j)] == b'a' {
                queue.push_back(Reverse((0, (i, j))));
                visited[(i, j)] = true;
            }
        }
    }
    queue.push_back(Reverse((0, map.start)));
    visited[map.start] = true;
    while let Some(Reverse((cost, (i, j)))) = queue.pop_front() {
        let height = map.heights[(i, j)];
        for neigh in map.heights.neighbors(i, j) {
            if visited[neigh] || map.heights[neigh] > height + 1 {
                continue;
            }
            if neigh == map.end {
                return Some(cost + 1);
            }
            visited[neigh] = true;
            queue.push_back(Reverse((cost + 1, neigh)));
        }
    }
    None
}

fn run(input: &str) -> u32 {
    let mut tokenizer = Tokenizer::new(input.as_bytes());
    let map = parse_heights(&mut tokenizer).unwrap();
    find_shortest_path(&map).unwrap()
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
