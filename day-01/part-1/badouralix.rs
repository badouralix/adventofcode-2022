use std::cmp;
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
    let split = input.split('\n');
    let mut max = 0;
    let mut sum = 0;

    for line in split {
        if line.is_empty() {
            max = cmp::max(max, sum);
            sum = 0;
            continue;
        }

        sum += line.parse::<isize>().unwrap();
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"),
            24000
        )
    }
}
