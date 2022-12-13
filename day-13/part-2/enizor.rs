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
    let mut packets = Vec::new();
    let p1 = Packet::from_bytes("[[2]]".as_bytes());
    packets.push(p1.clone());
    let p2 = Packet::from_bytes("[[6]]".as_bytes());
    packets.push(p2.clone());
    for (i, l) in input.lines().enumerate() {
        match i % 3 {
            0 | 1 => packets.push(Packet::from_bytes(l.as_bytes())),
            _ => {}
        }
    }
    packets.sort_unstable();
    let i1 = packets.binary_search(&p1).unwrap();
    let i2 = packets.binary_search(&p2).unwrap();
    (i1 + 1) * (i2 + 1)
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
