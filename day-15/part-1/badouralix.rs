use std::collections::HashSet;
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"), 2_000_000);
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str, y: isize) -> isize {
    let mut beacons = HashSet::new();
    let mut sensors = Vec::new();
    let mut distances = Vec::new();

    let mut min = 0;
    let mut max = 0;

    for line in input.lines() {
        let line = line.replace(['=', ',', ':'], " ");
        let split: Vec<&str> = line.split_whitespace().collect();
        let sensor = (split[3].parse().unwrap(), split[5].parse().unwrap());
        let beacon = (split[11].parse().unwrap(), split[13].parse().unwrap());

        beacons.insert(beacon);
        sensors.push(sensor);
        distances.push(distance(sensor, beacon));

        min = isize::min(
            min,
            1 + sensor.0 - distances[distances.len() - 1] as isize
                + isize::abs_diff(y, sensor.1) as isize,
        );
        max = isize::max(
            max,
            1 + sensor.0 + distances[distances.len() - 1] as isize
                - isize::abs_diff(y, sensor.1) as isize,
        );
    }

    let mut result = 0;
    let mut x = min;

    while x <= max {
        let beacon = (x, y);
        for (idx, &sensor) in sensors.iter().enumerate() {
            if distance(sensor, beacon) <= distances[idx] && !beacons.contains(&beacon) {
                result += 2
                    + sensor.0
                    + (distances[idx] as isize - isize::abs_diff(y, sensor.1) as isize)
                    - x;
                x = 1
                    + sensor.0
                    + (distances[idx] as isize - isize::abs_diff(y, sensor.1) as isize);
                if idx != 0 {
                    sensors.swap(0, idx);
                    distances.swap(0, idx);
                }
                break;
            }
        }
        x += 1;
    }

    result - 1
}

fn distance(sensor: (isize, isize), beacon: (isize, isize)) -> usize {
    isize::abs_diff(sensor.0, beacon.0) + isize::abs_diff(sensor.1, beacon.1)
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
