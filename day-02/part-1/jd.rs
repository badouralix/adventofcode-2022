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
    input.lines().fold(0, |acc, line| {
        let mut iter = line.chars();
        let opponent_move = char_to_score(iter.next().unwrap_or_default());
        iter.next();
        let player_move = char_to_score(iter.next().unwrap_or_default());

        acc + player_move + compare_scores(opponent_move, player_move)
    })
}

fn char_to_score(c: char) -> isize {
    match c {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => 0,
    }
}

fn compare_scores(a: isize, b: isize) -> isize {
    match b - a {
        0 => 3,
        -2 | 1 => 6,
        _ => 0,
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
            15
        )
    }
}
