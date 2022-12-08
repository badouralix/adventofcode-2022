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
    input
        .split("\n\n")
        .map(get_elve_calories)
        .max()
        .unwrap_or_default()
}

fn get_elve_calories(s: &str) -> isize {
    s.lines()
        .map(|y| y.parse::<isize>().unwrap_or_default())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
                .trim()),
            24000
        )
    }
}
