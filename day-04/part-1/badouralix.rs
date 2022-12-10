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
        let (left, right) = match line.split_once(',') {
            None => continue,
            Some((left, right)) => (left, right),
        };

        let (left_min, left_max): (i64, i64) = match left.split_once('-') {
            None => continue,
            Some((min, max)) => (min.parse().unwrap(), max.parse().unwrap()),
        };

        let (right_min, right_max): (i64, i64) = match right.split_once('-') {
            None => continue,
            Some((min, max)) => (min.parse().unwrap(), max.parse().unwrap()),
        };

        if (left_min <= right_min && right_max <= left_max)
            || (right_min <= left_min && left_max <= right_max)
        {
            result += 1;
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
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
                .trim()),
            2
        )
    }
}
