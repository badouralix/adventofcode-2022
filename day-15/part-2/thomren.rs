use std::env::args;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"), 0, 4000000);
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str, min: isize, max: isize) -> isize {
    let sensors = input
        .lines()
        .map(Sensor::from_str)
        .collect::<Result<Vec<Sensor>, _>>()
        .expect("Failed to parse input");

    for y in min..=max {
        let mut covered_ranges: Vec<(isize, isize)> =
            sensors.iter().filter_map(|s| s.covered_xslice(y)).collect();
        covered_ranges.sort();
        let merged_ranges = merge_ranges(&covered_ranges);

        let covered = merged_ranges
            .iter()
            .filter(|&&(from, to)| from >= min || to <= max)
            .count();
        // there is a non-covered spot iff there is more than one contiguous covered range
        if covered > 1 {
            let ranges = merged_ranges
                .into_iter()
                .filter(|&(from, to)| from >= min || to <= max)
                .collect::<Vec<(isize, isize)>>();
            // the end of the first range + 1 is not covered (else it would have been merged with the following range)
            let x = ranges[0].1 + 1;
            return tuning_frequency(x, y);
        }
    }

    0
}

fn merge_ranges(ranges: &Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    let mut merged = vec![];
    if ranges.is_empty() {
        return merged;
    }
    merged.push(ranges[0]);

    for &(from, to) in ranges[1..].iter() {
        let &(_, last_to) = merged.last().unwrap();
        if from <= last_to + 1 && to >= last_to {
            merged.last_mut().unwrap().1 = to;
        } else if from > last_to {
            merged.push((from, to));
        }
    }

    merged
}

#[derive(Debug)]
struct Sensor {
    x: isize,
    y: isize,
    closest_beacon: (isize, isize),
}

impl FromStr for Sensor {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.trim().split(' ').collect::<Vec<&str>>();
        if tokens.len() < 10 {
            return Err(Box::from("not enough tokens"));
        }
        let x = tokens[2][2..tokens[2].len() - 1].parse::<isize>().unwrap();
        let y = tokens[3][2..tokens[3].len() - 1].parse::<isize>().unwrap();
        let bx = tokens[8][2..tokens[8].len() - 1].parse::<isize>().unwrap();
        let by = tokens[9][2..].parse::<isize>().unwrap();

        Ok(Self {
            x,
            y,
            closest_beacon: (bx, by),
        })
    }
}

impl Sensor {
    fn range(&self) -> usize {
        manhattan_dist((self.x, self.y), self.closest_beacon)
    }

    fn covered_xslice(&self, y: isize) -> Option<(isize, isize)> {
        let range = self.range() as isize;
        let radius = range - (y - self.y).abs();
        if radius < 0 {
            return None;
        }
        Some((self.x - radius, self.x + radius))
    }
}

fn manhattan_dist(p: (isize, isize), q: (isize, isize)) -> usize {
    ((p.0 - q.0).abs() + (p.1 - q.1).abs()) as usize
}

fn tuning_frequency(x: isize, y: isize) -> isize {
    4000000 * x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run(
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
                0,
                20
            ),
            56000011
        )
    }
}
