use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> isize {
    const N: usize = 4;

    let mut total = 0;
    let mut chunks = input.as_bytes().chunks_exact(N);
    for round in &mut chunks {
        let round: [u8; N] = unsafe { round.try_into().unwrap_unchecked() };
        total += score(round) as isize;
    }
    let mut last_round = [0; 4];
    last_round[..chunks.remainder().len()].copy_from_slice(chunks.remainder());
    total += score(last_round) as isize;

    total
}

const MATCHUP_SCORE: [u8; 5] = [6, 0, 3, 6, 0];

fn score(round: [u8; 4]) -> u8 {
    let result_score =
        unsafe { MATCHUP_SCORE.get_unchecked((round[2] - b'X' + 2 - (round[0] - b'A')) as usize) };
    result_score + (round[2] - b'X') + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("A Y
B X
C Z"),
            15
        )
    }
}
