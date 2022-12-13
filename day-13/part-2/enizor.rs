use std::cmp::*;
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

#[derive(Clone)]
enum Packet {
    Scalar(usize),
    List(Vec<Packet>),
}

impl Packet {
    fn from_str(s: &[u8]) -> Self {
        // dbg!(s);
        if s.is_empty() {
            Self::List(Vec::new())
        } else if s[0] == b'[' {
            let mut res = Vec::new();
            let mut depth = 1;
            let mut cur1 = 1;
            let mut cur2 = 1;
            while cur2 < s.len() {
                match s[cur2] {
                    b'[' => depth += 1,
                    b']' => {
                        depth -= 1;
                        if depth == 0 {
                            res.push(Packet::from_str(&s[cur1..cur2]));
                            cur1 = cur2 + 1;
                        }
                    }
                    b',' => {
                        if depth == 1 {
                            res.push(Packet::from_str(&s[cur1..cur2]));
                            cur1 = cur2 + 1;
                        }
                    }
                    _ => {}
                }
                cur2 += 1;
            }
            Self::List(res)
        } else {
            let mut v = 0;
            for &b in s {
                v *= 10;
                v += (b - b'0') as usize;
            }
            Self::Scalar(v)
        }
    }

    fn impl_cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Scalar(x), Self::Scalar(y)) => x.cmp(y),
            (Self::Scalar(x), Self::List(_)) => {
                let w = Self::List(vec![Self::Scalar(*x)]);
                w.impl_cmp(other)
            }
            (Self::List(_), Self::Scalar(y)) => {
                let w = Self::List(vec![Self::Scalar(*y)]);
                self.impl_cmp(&w)
            }
            (Self::List(v), Self::List(w)) => {
                if v.is_empty() && w.is_empty() {
                    Ordering::Equal
                } else if v.is_empty() {
                    Ordering::Less
                } else if w.is_empty() {
                    Ordering::Greater
                } else {
                    let a = &v[0];
                    let b = &w[0];
                    let c = a.impl_cmp(b);
                    if c == Ordering::Equal {
                        let e = Self::List(v[1..].into());
                        let f = Self::List(w[1..].into());
                        e.impl_cmp(&f)
                    } else {
                        c
                    }
                }
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.impl_cmp(other)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.impl_cmp(other))
    }
}

impl Eq for Packet {}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.impl_cmp(other) == Ordering::Equal
    }
}

fn run(input: &str) -> usize {
    let mut packets = Vec::new();
    let p1 = Packet::from_str("[[2]]".as_bytes());
    packets.push(p1.clone());
    let p2 = Packet::from_str("[[6]]".as_bytes());
    packets.push(p2.clone());
    for (i, l) in input.lines().enumerate() {
        match i % 3 {
            0 | 1 => packets.push(Packet::from_str(l.as_bytes())),
            _ => {}
        }
    }
    packets.sort_unstable();
    // for p in &packets {
    //     println!("{}", p.to_string());
    // };
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
