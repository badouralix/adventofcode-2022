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
    let mut result = 0;

    for line in input.split('\n') {
        let opponent = line.chars().next().unwrap() as isize - 'A' as isize + 1;
        let outcome = line.chars().nth(2).unwrap();

        match outcome {
            'X' => result += 3 + (opponent - 4) % 3,
            'Y' => result += 3 + opponent,
            'Z' => result += 7 + opponent % 3,
            _ => todo!(),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("
A Y
B X
C Z
        "
            .trim()),
            12
        )
    }
}
