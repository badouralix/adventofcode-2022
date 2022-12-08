fn main() {
    aoc::run(run)
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

    let mut score = 0;

    for row in 1..size - 1 {
        for col in 1..size - 1 {
            let tree = map[row][col];

            let mut left_score = 0;
            for j in (0..col).rev() {
                left_score += 1;
                if tree <= map[row][j] {
                    break;
                }
            }

            let mut right_score = 0;
            for j in col + 1..size {
                right_score += 1;
                if tree <= map[row][j] {
                    break;
                }
            }

            let mut top_score = 0;
            for i in (0..row).rev() {
                top_score += 1;
                if tree <= map[i][col] {
                    break;
                }
            }

            let mut bottom_score = 0;
            for i in row + 1..size {
                bottom_score += 1;
                if tree <= map[i][col] {
                    break;
                }
            }

            let tree_score = left_score * right_score * top_score * bottom_score;

            if tree_score > score {
                score = tree_score;
            }
        }
    }

    score
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
            8
        )
    }
}
