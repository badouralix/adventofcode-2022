use std::cmp::{max, min};
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

fn run(input: &str) -> isize {
    let mut cubes: HashSet<[isize; 3]> = HashSet::with_capacity(3_000);
    let mut water: HashSet<[isize; 3]> = HashSet::with_capacity(12_000);
    let mut idx_min = 0;
    let mut idx_max = 0;

    for line in input.lines() {
        let mut split = line.splitn(3, ',');

        let x = split.next().unwrap().parse().unwrap();
        idx_min = min(idx_min, x);
        idx_max = max(idx_max, x);

        let y = split.next().unwrap().parse().unwrap();
        idx_min = min(idx_min, y);
        idx_max = max(idx_max, y);

        let z = split.next().unwrap().parse().unwrap();
        idx_min = min(idx_min, z);
        idx_max = max(idx_max, z);

        cubes.insert([x, y, z]);
    }

    let mut result = 0;
    let mut stack = vec![[0, 0, 0]];

    while let Some(cube) = stack.pop() {
        if water.contains(&cube) {
            continue;
        }

        if cubes.contains(&cube) {
            result += 1;
            continue;
        }

        water.insert(cube);

        for dimension in 0..cube.len() {
            for direction in [-1, 1] {
                let mut neighbor = cube;
                neighbor[dimension] += direction;

                if neighbor[dimension] >= idx_min - 1 && neighbor[dimension] <= idx_max + 1 {
                    stack.push(neighbor);
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_simple_test() {
        assert_eq!(
            run("
1,1,1
2,1,1
"
            .trim()),
            10
        )
    }

    #[test]
    fn run_larger_test() {
        assert_eq!(
            run("
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
"
            .trim()),
            58
        )
    }
}
