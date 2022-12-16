use std::collections::HashSet;
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"), 4_000_000);
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str, max: isize) -> isize {
    let mut beacons = HashSet::new();
    let mut sensors = Vec::new();
    let mut distances = Vec::new();

    for line in input.lines() {
        let line = line.replace(['=', ',', ':'], " ");
        let split: Vec<&str> = line.split_whitespace().collect();
        let sensor = (split[3].parse().unwrap(), split[5].parse().unwrap());
        let beacon = (split[11].parse().unwrap(), split[13].parse().unwrap());

        beacons.insert(beacon);
        sensors.push(sensor);
        distances.push(distance(sensor, beacon));
    }

    let mut x = 0;
    let mut y;
    let mut beacon;
    while x <= max {
        y = 0;
        'next: while y <= max {
            beacon = (x, y);
            for (idx, &sensor) in sensors.iter().enumerate() {
                if distance(sensor, beacon) <= distances[idx] {
                    y = 1 + isize::max(
                        y,
                        sensor.1
                            + (distances[idx] as isize - isize::abs_diff(x, sensor.0) as isize),
                    );

                    if idx != 0 {
                        // sensors.swap(0, idx);
                        sensors[0..=idx].rotate_right(1);
                        // distances.swap(0, idx);
                        distances[0..=idx].rotate_right(1);
                        // println!("{:?}", sensors);
                    }
                    continue 'next;
                }
            }
            if beacons.contains(&beacon) {
                y += 1;
                continue;
            }
            return x * 4_000_000 + y;
        }

        x += 1;
    }

    unreachable!()
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
                20
            ),
            56000011
        )
    }
}
