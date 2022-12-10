use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

const SIZE: usize = 99;

fn run(input: &str) -> usize {
    process(input, SIZE)
}

fn process(input: &str, size: usize) -> usize {
    let mut map = [[0; SIZE]; SIZE];

    for (row, line) in input.lines().enumerate() {
        for (col, tree) in line.chars().enumerate() {
            let height = tree.to_digit(10).unwrap_or_default() as usize;
            map[row][col] = height;
        }
    }

    let mut visible_trees = 4 * size - 4;

    for row in 1..size - 1 {
        for col in 1..size - 1 {
            let tree = map[row][col];

            let left = map[row][..col].iter().max().copied().unwrap_or_default();
            let right = map[row][col + 1..]
                .iter()
                .max()
                .copied()
                .unwrap_or_default();

            let mut top = 0;
            for l in map.iter().take(row) {
                let e = l[col];
                if e > top {
                    top = e;
                }
            }

            let mut bottom = 0;
            for l in map.iter().take(size).skip(row + 1) {
                let e = l[col];
                if e > bottom {
                    bottom = e;
                }
            }

            if tree > left || tree > right || tree > top || tree > bottom {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            process(
                "30373
25512
65332
33549
35390",
                5
            ),
            21
        )
    }
}
