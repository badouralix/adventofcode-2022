use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

#[allow(clippy::needless_range_loop)]
fn run(input: &str) -> usize {
    let trees = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&x| x - b'0')
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    let (height, width) = (trees.len(), trees[0].len());

    (0..height)
        .flat_map(|i| (0..width).map(move |j| (i, j)))
        .map(|pos| scenic_score(&trees, pos))
        .max()
        .unwrap_or_default()
}

fn scenic_score(trees: &Vec<Vec<u8>>, pos: (usize, usize)) -> usize {
    let (height, width) = (trees.len() as isize, trees[0].len() as isize);
    let mut res = 1;
    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let mut i = pos.0 as isize;
        let mut j = pos.1 as isize;
        let mut distance = 0;
        loop {
            i += dx;
            j += dy;
            if i < 0 || j < 0 || i >= height || j >= width {
                break;
            }
            distance += 1;
            if trees[i as usize][j as usize] >= trees[pos.0][pos.1] {
                break;
            };
        }
        res *= distance;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("
30373
25512
65332
33549
35390".trim()), 8)
    }
}
