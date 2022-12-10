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
        let compartment = line[..line.len() / 2].as_bytes();
        for item in line[line.len() / 2..].as_bytes() {
            if compartment.contains(item) {
                match item {
                    b'a'..=b'z' => result += (*item - b'a' + 1) as isize,
                    b'A'..=b'Z' => result += (*item - b'A' + 27) as isize,
                    _ => todo!(),
                }
                break;
            }
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
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
                .trim()),
            157
        )
    }
}
