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
    input.lines().map(compute_score).sum()
}

fn compute_score(line:  &str) -> usize {
    let line = line.as_bytes();
    let opponent_move = line[0] - b'A';
    let outcome = line[2] - b'X';
    let elve_move = match outcome {
        0 => (opponent_move as isize - 1).rem_euclid(3),
        1 => opponent_move as isize,
        2 => (opponent_move as isize + 1) % 3,
        _ => panic!("invalid outcome: {}", line[2])
    } as usize;
    3 * outcome as usize + elve_move + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("
A Y
B X
C Z".trim()), 12);

assert_eq!(run("
A X".trim()), 3);
    }
}
