const SIZE: usize = 14;

fn main() {
    aoc::run(run)
}

fn run(input: &str) -> usize {
    // Your code goes here
    let mut i = 0;

    'outer: while i < input.len() - 4 {
        for j in (1..SIZE).rev() {
            if input.as_bytes()[i + j..i + SIZE].contains(&input.as_bytes()[i + j - 1]) {
                i += j;
                continue 'outer;
            }
        }

        return i + SIZE;
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(run("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(run("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
