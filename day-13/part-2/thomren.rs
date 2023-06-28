use std::cmp::Ordering;
use std::env::args;
use std::slice::Iter;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> usize {
    let mut packets = input
        .lines()
        .filter(|&line| !line.is_empty())
        .map(Packet::from_str)
        .collect::<Result<Vec<Packet>, _>>()
        .expect("Failed to parse packet");

    let delimiter1 = Packet::from_str("[[2]]").unwrap();
    let delimiter2 = Packet::from_str("[[6]]").unwrap();
    packets.push(delimiter1.clone());
    packets.push(delimiter2.clone());

    packets.sort();
    (packets.iter().position(|x| *x == delimiter1).unwrap() + 1)
        * (packets.iter().position(|x| *x == delimiter2).unwrap() + 1)
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    Int(usize),
    Data(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let mut i = 0;
        let mut stack: Vec<Packet> = Vec::new();
        while i < bytes.len() {
            let b = bytes[i];
            if b == b'[' {
                stack.push(Packet::Data(vec![]));
            } else if b == b']' {
                let p = stack.pop().unwrap();
                if !stack.is_empty() {
                    let top = stack.last_mut().unwrap();
                    if let Packet::Data(v) = top {
                        v.push(p);
                    } else {
                        return Err(Box::from("Cannot parse packet"));
                    }
                } else {
                    return Ok(p);
                }
            } else if b.is_ascii_digit() {
                let (n, n_bytes) = atoi(&mut bytes[i..].iter());
                i += n_bytes - 1;
                if let Packet::Data(v) = stack.last_mut().unwrap() {
                    v.push(Packet::Int(n));
                } else {
                    return Err(Box::from("Missing opening bracket"));
                }
            }
            i += 1;
        }
        Err(Box::from("Missing closing bracket"))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (&Int(x), &Int(y)) => x.cmp(&y),
            (&Int(x), Data(_)) => Data(vec![Int(x)]).cmp(other),
            (Data(_), &Int(y)) => self.cmp(&Data(vec![Int(y)])),
            (Data(a), Data(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Parse a number from a bytes iterator, stopping when a
/// non-digit character is encountered
fn atoi(it: &mut Iter<u8>) -> (usize, usize) {
    let mut res = 0;
    let mut n_bytes = 0;
    for &b in it {
        match b {
            b'0'..=b'9' => {}
            _ => break,
        }
        res *= 10;
        res += (b - b'0') as usize;
        n_bytes += 1;
    }
    (res, n_bytes)
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
