fn main() {
    aoc::run(run)
}

fn run(input: &str) -> usize {
    // Your code goes here
    let mut i = 0;

    while i < input.len() - 4 {
        if input.as_bytes()[i + 3..i + 4].contains(&input.as_bytes()[i + 2]) {
            i += 3;
            continue;
        }

        if input.as_bytes()[i + 2..i + 4].contains(&input.as_bytes()[i + 1]) {
            i += 2;
            continue;
        }

        if input.as_bytes()[i + 1..i + 4].contains(&input.as_bytes()[i]) {
            i += 1;
            continue;
        }

        return i + 4;
    }

    todo!()
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
