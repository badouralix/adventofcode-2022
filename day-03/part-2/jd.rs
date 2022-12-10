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
    let lines = input.lines().collect::<Vec<&str>>();

    lines
        .chunks_exact(3)
        .fold(0, |acc, group| acc + process_group(group))
}

fn process_group(group: &[&str]) -> usize {
    for f in group[0].chars() {
        if group[1].contains(f) && group[2].contains(f) {
            return priority(f);
        }
    }

    0
}

fn priority(c: char) -> usize {
    let code = c as usize;
    if code >= 97 {
        code - 97 + 1
    } else {
        code - 65 + 26 + 1
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
            70
        )
    }
}
