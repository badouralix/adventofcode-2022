use std::env::args;
use std::time::Instant;

use aoc::paullgdc::{matrix::Matrix, tokenize::Tokenizer};

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn parse_grid(tokenizer: &mut Tokenizer) -> Matrix<u8> {
    let mut trees = Vec::with_capacity(100);
    let mut row_len;
    'line: loop {
        row_len = 0;
        while let Some(v) = tokenizer.next_ascii_char() {
            if v == b'\n' {
                continue 'line;
            }
            trees.push(v - b'0');
            row_len += 1;
        }
        break;
    }

    Matrix::from_vec(trees, row_len).unwrap()
}

fn run(input: &str) -> isize {
    let grid = parse_grid(&mut Tokenizer::new(input.as_bytes()));
    let mut visible = grid.map(|_| false);
    for i in 0..visible.dims.0 {
        *visible.get_mut(i, 0).unwrap() = true;
        *visible.get_mut(i, visible.dims.1 - 1).unwrap() = true;
    }
    for j in 0..visible.dims.1 {
        *visible.get_mut(0, j).unwrap() = true;
        *visible.get_mut(visible.dims.0 - 1, j).unwrap() = true;
    }
    for i in 0..grid.dims.0 {
        let mut highest = 0;
        for j in 0..grid.dims.1 {
            let tree = *grid.get(i, j).unwrap();
            if tree > highest {
                *visible.get_mut(i, j).unwrap() = true;
                highest = tree;
            }
        }
    }
    for i in 0..grid.dims.0 {
        let mut highest = 0;
        for j in (0..grid.dims.1).rev() {
            let tree = *grid.get(i, j).unwrap();
            if tree > highest {
                *visible.get_mut(i, j).unwrap() = true;
                highest = tree;
            }
        }
    }
    for j in 0..grid.dims.1 {
        let mut highest = 0;
        for i in 0..grid.dims.0 {
            let tree = *grid.get(i, j).unwrap();
            if tree > highest {
                *visible.get_mut(i, j).unwrap() = true;
                highest = tree;
            }
        }
    }
    for j in 0..grid.dims.1 {
        let mut highest = 0;
        for i in (0..grid.dims.0).rev() {
            let tree = *grid.get(i, j).unwrap();
            if tree > highest {
                *visible.get_mut(i, j).unwrap() = true;
                highest = tree;
            }
        }
    }
    visible.iter().filter(|&&v| v).count() as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("30373
25512
65332
33549
35390"),
            21
        )
    }
}
