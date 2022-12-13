use std::cmp::*;
use std::fmt;
use std::fmt::Write;

#[derive(Clone)]
pub enum Packet {
    Scalar(usize),
    List(Vec<Packet>),
}

impl Packet {
    pub fn from_bytes(s: &[u8]) -> Self {
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
                            res.push(Packet::from_bytes(&s[cur1..cur2]));
                            cur1 = cur2 + 1;
                        }
                    }
                    b',' => {
                        if depth == 1 {
                            res.push(Packet::from_bytes(&s[cur1..cur2]));
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
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Scalar(x), Self::Scalar(y)) => x.cmp(y),
            (Self::Scalar(x), Self::List(_)) => {
                let w = Self::List(vec![Self::Scalar(*x)]);
                w.cmp(other)
            }
            (Self::List(_), Self::Scalar(y)) => {
                let w = Self::List(vec![Self::Scalar(*y)]);
                self.cmp(&w)
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
                    let c = a.cmp(b);
                    if c == Ordering::Equal {
                        let e = Self::List(v[1..].into());
                        let f = Self::List(w[1..].into());
                        e.cmp(&f)
                    } else {
                        c
                    }
                }
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Packet {}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scalar(x) => f.write_str(&x.to_string())?,
            Self::List(v) => {
                f.write_char('[')?;
                let mut first = true;
                for p in v {
                    if !first {
                        f.write_char(',')?;
                    } else {
                        first = false;
                    }
                    p.fmt(f)?;
                }
                f.write_char(']')?
            }
        }
        Ok(())
    }
}

impl std::str::FromStr for Packet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_bytes(s.as_bytes()))
    }
}

mod test {
    #[test]
    fn test() -> Result<(), ()> {
        use super::Packet;
        let p1 = "[1,1,3,1,1]".parse::<Packet>()?;
        let p2 = "[1,1,5,1,1]".parse::<Packet>()?;

        assert_eq!(p1, p1.clone());
        assert!(p1 < p2);
        assert!(p2 > p1);
        assert_eq!(format!("{:?}", p1), "[1,1,3,1,1]");
        let p3 = "[[1],[2,3,4]]".parse::<Packet>()?;
        let p4 = "[[1],4]".parse::<Packet>()?;
        assert!(p3 < p4);
        assert!(p4 > p3);
        let p5 = "[9]".parse::<Packet>()?;
        let p6 = "[[8,7,6]]".parse::<Packet>()?;
        assert!(p5 > p6);
        assert!(p6 < p5);
        let p7 = "[[4,4],4,4]".parse::<Packet>()?;
        let p8 = "[[4,4],4,4,4]".parse::<Packet>()?;
        assert!(p7 < p8);
        assert!(p8 > p7);
        let p9 = "[7,7,7,7]".parse::<Packet>()?;
        let p10 = "[7,7,7]".parse::<Packet>()?;
        assert!(p9 > p10);
        assert!(p10 < p9);
        let p11 = "[]".parse::<Packet>()?;
        let p12 = "[3]".parse::<Packet>()?;
        assert!(p11 < p12);
        assert!(p12 > p11);
        let p13 = "[[[]]]".parse::<Packet>()?;
        let p14 = "[[]]".parse::<Packet>()?;
        assert!(p13 > p14);
        assert!(p14 < p13);
        let p15 = "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse::<Packet>()?;
        let p16 = "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse::<Packet>()?;
        assert_eq!(format!("{:?}", p16), "[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert!(p15 > p16);
        assert!(p16 < p15);
        Ok(())
    }
}
