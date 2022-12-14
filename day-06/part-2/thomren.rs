use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

const MARKER_SIZE: usize = 14;

fn run(input: &str) -> usize {
    let input = input.as_bytes();
    for i in 0..(input.len() - MARKER_SIZE) {
        if all_unique(&input[i..(i + MARKER_SIZE)]) {
            return i + MARKER_SIZE;
        }
    }
    0
}

fn all_unique(a: &[u8]) -> bool {
    for i in 0..a.len() {
        for j in (i + 1)..a.len() {
            if a[i] == a[j] {
                return false;
            }
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
