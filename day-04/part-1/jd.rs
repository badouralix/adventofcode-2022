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
    input.lines().fold(0, |acc, line| acc + full_overlap(line))
}

fn full_overlap(line: &str) -> usize {
    let ((a, b), (c, d)) = parse_ranges(line);

    usize::from((a >= c && b <= d) || (a <= c && b >= d))
}

fn parse_ranges(ranges: &str) -> ((usize, usize), (usize, usize)) {
    let intervals = ranges.split(',').collect::<Vec<&str>>();

    (parse_range(intervals[0]), parse_range(intervals[1]))
}

fn parse_range(range: &str) -> (usize, usize) {
    let interval = range
        .split('-')
        .map(|e| e.parse::<usize>().unwrap_or_default())
        .collect::<Vec<usize>>();

    (interval[0], interval[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"),
            2
        )
    }
}
