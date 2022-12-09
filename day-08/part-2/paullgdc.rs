use aoc::paullgdc::{matrix::Matrix, tokenize::Tokenizer};

fn main() {
    aoc::run(run)
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

fn compute_visibility(trees: &Matrix<u8>) -> u32 {
    let mut max_vis = 0;
    for j in 0..trees.dims.1 {
        for i in 0..trees.dims.0 {
            let tree = trees[(i, j)];
            let mut total_visibility = 1;
            let mut vis;

            vis = 0;
            for k in i + 1..trees.dims.0 {
                let neighbor = trees[(k, j)];
                vis += 1;
                if neighbor >= tree {
                    break;
                }
            }
            total_visibility *= vis;

            vis = 0;
            for k in (0..i).rev() {
                let neighbor = trees[(k, j)];
                vis += 1;
                if neighbor >= tree {
                    break;
                }
            }
            total_visibility *= vis;

            vis = 0;
            for k in j + 1..trees.dims.1 {
                let neighbor = trees[(i, k)];
                vis += 1;
                if neighbor >= tree {
                    break;
                }
            }
            total_visibility *= vis;

            vis = 0;
            for k in (0..j).rev() {
                let neighbor = trees[(i, k)];
                vis += 1;
                if neighbor >= tree {
                    break;
                }
            }
            total_visibility *= vis;

            max_vis = std::cmp::max(max_vis, total_visibility);
        }
    }
    max_vis
}

fn run(input: &str) -> u32 {
    let trees = parse_grid(&mut Tokenizer::new(input.as_bytes()));
    compute_visibility(&trees)
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
            8
        )
    }
}
