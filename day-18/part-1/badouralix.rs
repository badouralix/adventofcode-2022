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

    for line in input.lines() {
        let mut split = line.splitn(3, ',');
        cubes.insert([
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        ]);
    }

    let mut result = 0;

    for cube in &cubes {
        for dimension in 0..cube.len() {
            for direction in [-1, 1] {
                let mut neighbor = *cube;
                neighbor[dimension] += direction;

                if !cubes.contains(&neighbor) {
                    result += 1;
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
            64
        )
    }
}
