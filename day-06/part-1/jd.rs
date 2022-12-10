use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> usize {
    let mut buffer = [0 as char; 4];
    let mut cursor = 0;

    for (i, c) in input.chars().take(4).enumerate() {
        buffer[i] = c;
    }

    for (i, c) in input.chars().enumerate() {
        if unique(&buffer) {
            return i;
        }

        buffer[cursor] = c;

        cursor = (cursor + 1) % 4;
    }

    0
}

fn unique(buffer: &[char; 4]) -> bool {
    buffer[0] != buffer[1]
        && buffer[0] != buffer[2]
        && buffer[0] != buffer[3]
        && buffer[1] != buffer[2]
        && buffer[1] != buffer[3]
        && buffer[2] != buffer[3]
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
