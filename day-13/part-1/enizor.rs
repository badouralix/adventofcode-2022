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
    let mut res = 0;
    let mut p1 = Packet::new("");
    let mut p2: Packet;
    let mut k = 1;
    for (i, l) in input.lines().enumerate() {
        match i % 3 {
            0 => p1 = Packet::new(l),
            1 => {
                p2 = Packet::new(l);
                if p1 < p2 {
                    res += k;
                }
                k += 1;
            }
            _ => {}
        }
    }
    res
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
            13
        )
    }
}
