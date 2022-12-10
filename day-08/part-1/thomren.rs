use std::collections::HashSet;
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

    let mut visible_trees = HashSet::<(usize, usize)>::new();

    for i in 0..height {
        let mut max_size = None;
        for j in 0..width {
            if max_size.is_none() || trees[i][j] > max_size.unwrap() {
                visible_trees.insert((i, j));
                max_size = Some(trees[i][j]);
            }
        }
    }

    for j in 0..width {
        let mut max_size = None;
        for i in 0..height {
            if max_size.is_none() || trees[i][j] > max_size.unwrap() {
                visible_trees.insert((i, j));
                max_size = Some(trees[i][j]);
            }
        }
    }

    for i in (0..height).rev() {
        let mut max_size = None;
        for j in (0..width).rev() {
            if max_size.is_none() || trees[i][j] > max_size.unwrap() {
                visible_trees.insert((i, j));
                max_size = Some(trees[i][j]);
            }
        }
    }

    for j in (0..width).rev() {
        let mut max_size = None;
        for i in (0..height).rev() {
            if max_size.is_none() || trees[i][j] > max_size.unwrap() {
                visible_trees.insert((i, j));
                max_size = Some(trees[i][j]);
            }
        }
    }

    visible_trees.len()
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
35390".trim()), 21)
    }
}
