use std::collections::{HashMap, HashSet};
use std::env::args;
use std::time::Instant;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Beacon {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Sensor {
    x: isize,
    y: isize,
}

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"), 2_000_000);
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str, y: isize) -> isize {
    let mut beacons = HashSet::new();
    let mut sensors = HashMap::new();

    let mut min = 0;
    let mut max = 0;

    for line in input.lines() {
        let line = line.replace(['=', ',', ':'], " ");
        let split: Vec<&str> = line.split_whitespace().collect();
        let sensor = Sensor {
            x: split[3].parse().unwrap(),
            y: split[5].parse().unwrap(),
        };
        let beacon = Beacon {
            x: split[11].parse().unwrap(),
            y: split[13].parse().unwrap(),
        };

        beacons.insert(beacon);
        sensors.insert(sensor, distance(sensor, beacon));

        min = isize::min(min, sensor.x - sensors[&sensor] as isize);
        max = isize::max(max, sensor.x + sensors[&sensor] as isize);
    }

    let mut result = 0;
    for x in min..=max {
        let beacon = Beacon { x, y };
        for &sensor in sensors.keys() {
            if distance(sensor, beacon) <= sensors[&sensor] && !beacons.contains(&beacon) {
                result += 1;
                break;
            }
        }
    }

    result
}

fn distance(sensor: Sensor, beacon: Beacon) -> usize {
    isize::abs_diff(sensor.x, beacon.x) + isize::abs_diff(sensor.y, beacon.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run(
                "
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
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
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
                    .trim(),
                10
            ),
            26
        )
    }
}
