use std::collections::HashSet;

fn main() {
    aoc::run(run)
}

#[allow(clippy::needless_range_loop)]
fn run(input: &str) -> usize {
    let mut history = HashSet::<(usize, usize)>::new();
    let split: Vec<&str> = input.split('\n').collect();

    for i in 0..split.len() {
        let mut max = -1;
        for j in 0..split.len() {
            let current = (split[i].as_bytes()[j] as i32) - (b'0' as i32);
            if current > max {
                history.insert((i, j));
                max = current
            }
        }
    }

    for i in 0..split.len() {
        let mut max = -1;
        for j in (0..split.len()).rev() {
            let current = (split[i].as_bytes()[j] as i32) - (b'0' as i32);
            if current > max {
                history.insert((i, j));
                max = current
            }
        }
    }

    for j in 0..split.len() {
        let mut max = -1;
        for i in 0..split.len() {
            let current = (split[i].as_bytes()[j] as i32) - (b'0' as i32);
            if current > max {
                history.insert((i, j));
                max = current
            }
        }
    }

    for j in 0..split.len() {
        let mut max = -1;
        for i in (0..split.len()).rev() {
            let current = (split[i].as_bytes()[j] as i32) - (b'0' as i32);
            if current > max {
                history.insert((i, j));
                max = current
            }
        }
    }

    history.len()
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
            21
        )
    }
}
