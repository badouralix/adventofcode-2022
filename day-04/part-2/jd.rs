fn main() {
    aoc::run(run)
}

fn run(input: &str) -> usize {
    input.lines().fold(0, |acc, line| acc + does_overlap(line))
}

fn does_overlap(line: &str) -> usize {
    let ((a, b), (c, d)) = parse_ranges(line);

    usize::from((c <= a && d >= a) || (a <= c && b >= c))
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
            4
        )
    }
}
