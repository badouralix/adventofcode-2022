use aoc::enizor::packet::Packet;

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
    let p1: Packet = "[[2]]".parse().unwrap();
    let p2: Packet = "[[6]]".parse().unwrap();
    let mut i1 = 1;
    let mut i2 = 2;
    for (i, l) in input.lines().enumerate() {
        if i % 3 != 2 {
            let p: Packet = l.parse().unwrap();
            if p < p1 {
                i1 += 1;
                i2 += 1;
            } else if p < p2 {
                i2 += 1;
            }
        }
    }
    i1 * i2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"),
            140
        )
    }
}
