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

fn compute_score(line: &str) -> usize {
    let line = line.as_bytes();
    let opponent_move = line[0] - b'A';
    let elve_move = line[2] - b'X';
    let outcome_score = match (opponent_move, elve_move) {
        (a, b) if a == b => 3,
        (a, b) if (a + 1) % 3 == b => 6,
        (_, _) => 0,
    };
    outcome_score + elve_move as usize + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("
A Y
B X
C Z".trim()), 15)
    }
}
