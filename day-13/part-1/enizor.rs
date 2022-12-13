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
    let mut res = 0;
    let mut p1 = Packet::Scalar(0);
    let mut p2: Packet;
    let mut k = 1;
    for (i, l) in input.lines().enumerate() {
        match i % 3 {
            0 => p1 = Packet::from_str(l.as_bytes()),
            1 => {
                p2 = Packet::from_str(l.as_bytes());
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
        let p1 = Packet::from_str("[1,1,3,1,1]".as_bytes());
        let p2 = Packet::from_str("[1,1,5,1,1]".as_bytes());

        assert!(p1 == p1);
        assert!(p1 < p2);
        assert!(p2 > p1);
        let p3 = Packet::from_str("[[1],[2,3,4]]".as_bytes());
        let p4 = Packet::from_str("[[1],4]".as_bytes());
        assert!(p3 < p4);
        assert!(p4 > p3);
        let p5 = Packet::from_str("[9]".as_bytes());
        let p6 = Packet::from_str("[[8,7,6]]".as_bytes());
        assert!(p5 > p6);
        assert!(p6 < p5);
        let p7 = Packet::from_str("[[4,4],4,4]".as_bytes());
        let p8 = Packet::from_str("[[4,4],4,4,4]".as_bytes());
        assert!(p7 < p8);
        assert!(p8 > p7);
        let p9 = Packet::from_str("[7,7,7,7]".as_bytes());
        let p10 = Packet::from_str("[7,7,7]".as_bytes());
        assert!(p9 > p10);
        assert!(p10 < p9);
        let p11 = Packet::from_str("[]".as_bytes());
        let p12 = Packet::from_str("[3]".as_bytes());
        assert!(p11 < p12);
        assert!(p12 > p11);
        let p13 = Packet::from_str("[[[]]]".as_bytes());
        let p14 = Packet::from_str("[[]]".as_bytes());
        assert!(p13 > p14);
        assert!(p14 < p13);
        let p15 = Packet::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]".as_bytes());
        let p16 = Packet::from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]".as_bytes());
        assert!(p15 > p16);
        assert!(p16 < p15);
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
