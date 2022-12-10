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
    input.lines().fold(0, |acc, bag| acc + process_bag(bag))
}

fn process_bag(line: &str) -> usize {
    let line = line.chars().collect::<Vec<char>>();

    for f in &line[..=line.len() / 2] {
        if line[line.len() / 2..].contains(f) {
            return priority(*f);
        }
    }

    0
}

fn priority(c: char) -> usize {
    let code = c as usize;
    if code >= 97 {
        code - 96
    } else {
        code - 38
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"),
            157
        )
    }
}
