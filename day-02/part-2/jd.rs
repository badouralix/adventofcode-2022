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
    input.lines().fold(0, |acc, line| {
        let mut iter = line.chars();
        let opponent_move = char_to_score(iter.next().unwrap_or_default());
        iter.next();
        let round_objective = char_to_score(iter.next().unwrap_or_default());

        acc + round_objective + player_move(opponent_move, round_objective)
    })
}

fn char_to_score(c: char) -> usize {
    match c {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    }
}

fn player_move(opponent_move: usize, round_objective: usize) -> usize {
    if round_objective == 3 {
        opponent_move
    } else if round_objective == 6 {
        (opponent_move % 3) + 1
    } else {
        ((opponent_move + 1) % 3) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("A Y
B X
C Z
"),
            12
        )
    }
}
