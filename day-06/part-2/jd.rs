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
    let mut buffer = [0 as char; 14];
    let mut cursor = 0;

    for (i, c) in input.chars().take(14).enumerate() {
        buffer[i] = c;
    }

    for (i, c) in input.chars().enumerate() {
        if unique(&buffer) {
            return i;
        }

        buffer[cursor] = c;

        cursor = (cursor + 1) % 14;
    }

    0
}

fn unique(buffer: &[char; 14]) -> bool {
    let mut set = [false; 26];

    for c in buffer {
        if set[*c as usize - 97] {
            return false;
        } else {
            set[*c as usize - 97] = true;
        }
    }

    true
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
