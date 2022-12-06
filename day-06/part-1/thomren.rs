use std::collections::HashSet;

fn main() {
    aoc::run(run)
}

const MARKER_SIZE: usize = 4;

fn run(input: &str) -> usize {
    let input = input.as_bytes();
    for i in 0..(input.len() - MARKER_SIZE) {
        if HashSet::<u8>::from_iter(input[i..i+MARKER_SIZE].iter().cloned()).len() == MARKER_SIZE {
            return i + MARKER_SIZE;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(run("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(run("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
