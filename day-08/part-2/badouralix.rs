fn main() {
    aoc::run(run)
}

#[allow(clippy::needless_range_loop)]
fn run(input: &str) -> isize {
    let mut result = 0;
    let split: Vec<&str> = input.split('\n').collect();

    for i in 1..(split.len() - 1) {
        for j in 1..(split.len() - 1) {
            let current = (split[i].as_bytes()[j] as i32) - (b'0' as i32);
            let mut score = 1;
            let mut trees: isize;

            // Look up
            trees = 0;
            for k in (0..i).rev() {
                trees += 1;
                if (split[k].as_bytes()[j] as i32) - (b'0' as i32) >= current {
                    break;
                }
            }
            score *= trees;

            // Look left
            trees = 0;
            for k in (0..j).rev() {
                trees += 1;
                if (split[i].as_bytes()[k] as i32) - (b'0' as i32) >= current {
                    break;
                }
            }
            score *= trees;

            // Look down
            trees = 0;
            for k in (i + 1)..split.len() {
                trees += 1;
                if (split[k].as_bytes()[j] as i32) - (b'0' as i32) >= current {
                    break;
                }
            }
            score *= trees;

            // Look right
            trees = 0;
            for k in (j + 1)..split.len() {
                trees += 1;
                if (split[i].as_bytes()[k] as i32) - (b'0' as i32) >= current {
                    break;
                }
            }
            score *= trees;

            if score > result {
                result = score;
            }
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
30373
25512
65332
33549
35390"
                .trim()),
            8
        )
    }
}
