use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"), 2000000);
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn parse_line(input: &str) -> (isize, isize, isize, isize) {
    // Sensor at x=9, y=16: closest beacon is at x=10, y=16
    let p1 = 12;
    let p2 = p1 + input[p1..].find(',').unwrap();
    let x1 = input[p1..p2].parse().unwrap();
    let p3 = p2 + 4;
    let p4 = p3 + input[p3..].find(':').unwrap();
    let y1 = input[p3..p4].parse().unwrap();
    let p5 = p4 + 25;
    let p6 = p5 + input[p5..].find(',').unwrap();
    let x2 = input[p5..p6].parse().unwrap();
    let y2 = input[p6 + 4..].parse().unwrap();
    (x1, y1, x2, y2)
}

fn manhattan_dist(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}

/// A collection of non-overlapping semi-open intervals [a, b[
#[derive(Debug, Default)]
struct IntervalCollection {
    intervals: Vec<(isize, isize)>,
}

impl IntervalCollection {
    fn add_interval(&mut self, mut a: isize, mut b: isize) {
        if a >= b {
            return;
        }
        for (c, d) in &self.intervals.clone() {
            if b <= *c || *d <= a {
                // intervals do not overlap
                continue;
            }
            match (*c <= a, b <= *d) {
                (true, true) => return, // fully covered
                (true, false) => a = *d,
                (false, true) => b = *c,
                (false, false) => {
                    self.add_interval(a, *c);
                    self.add_interval(*d, b);
                    return;
                }
            }
        }
        if a < b {
            self.intervals.push((a, b));
        }
    }
}

fn run(input: &str, y: isize) -> isize {
    let mut intervals = IntervalCollection::default();
    for line in input.lines() {
        let (x1, y1, x2, y2) = parse_line(line);
        let d = manhattan_dist(x1, y1, x2, y2);
        let dy = (y1 - y).abs();
        if dy <= d {
            let l = d - dy;
            if y2 != y {
                intervals.add_interval(x1 - l, x1 + l + 1);
            } else {
                intervals.add_interval(x1 - l, x2);
                intervals.add_interval(x2 + 1, x1 + l + 1);
            }
        }
    }
    let mut res = 0;
    for (a, b) in intervals.intervals {
        res += b - a;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run(
                "Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
                10
            ),
            26
        )
    }
}
