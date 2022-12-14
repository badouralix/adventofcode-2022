use std::collections::HashSet;
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

#[allow(unreachable_code)]
fn run(input: &str) -> usize {
    let mut bottom = 0;
    let mut scan = HashSet::new();
    for line in input.lines() {
        let coordinates: Vec<(usize, usize)> = line
            .split(" -> ")
            .map(|rock| rock.split_once(',').unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect();
        for i in 0..coordinates.len() - 1 {
            if coordinates[i].1 > bottom {
                bottom = coordinates[i].1
            }
            if coordinates[i + 1].1 > bottom {
                bottom = coordinates[i + 1].1
            }

            if coordinates[i].0 == coordinates[i + 1].0 {
                for y in usize::min(coordinates[i].1, coordinates[i + 1].1)
                    ..=usize::max(coordinates[i].1, coordinates[i + 1].1)
                {
                    scan.insert((coordinates[i].0, y));
                }
            } else if coordinates[i].1 == coordinates[i + 1].1 {
                for x in usize::min(coordinates[i].0, coordinates[i + 1].0)
                    ..=usize::max(coordinates[i].0, coordinates[i + 1].0)
                {
                    scan.insert((x, coordinates[i].1));
                }
            } else {
                unimplemented!();
            }
        }
    }

    bottom += 1;

    let mut result = 0;
    loop {
        let mut sand = (500, 0);
        loop {
            if scan.contains(&sand) {
                return result;
            }

            if sand.1 >= bottom {
                scan.insert(sand);
                result += 1;
                break;
            } else if !scan.contains(&(sand.0, sand.1 + 1)) {
                sand = (sand.0, sand.1 + 1);
            } else if !scan.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !scan.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                scan.insert(sand);
                result += 1;
                break;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
                .trim()),
            93
        )
    }
}
