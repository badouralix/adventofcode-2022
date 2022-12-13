use std::cmp::*;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Packet<'a> {
    bytes: &'a [u8],
}

impl<'a> Packet<'a> {
    fn get_first(&self) -> Option<(Self, bool, usize)> {
        if self.bytes.is_empty() {
            None
        } else if self.bytes[0] == b'[' {
            let mut cur = 1;
            let mut depth = 1;
            while cur < self.bytes.len() {
                match self.bytes[cur] {
                    b'[' => depth += 1,
                    b']' => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    _ => {}
                }
                cur += 1;
            }
            Some((
                Packet {
                    bytes: &self.bytes[1..cur],
                },
                false,
                cur + 1,
            ))
        } else if self.bytes[0].is_ascii_digit() {
            let mut cur = 1;
            while cur < self.bytes.len() {
                if !self.bytes[cur].is_ascii_digit() {
                    break;
                }
                cur += 1;
            }
            Some((
                Packet {
                    bytes: &self.bytes[0..cur],
                },
                true,
                cur,
            ))
        } else {
            panic!()
        }
    }

    fn split_first(&self) -> (Option<(Self, bool)>, Option<Self>) {
        match self.get_first() {
            None => (None, None),
            Some((p, b, cur)) => {
                if cur == self.bytes.len() {
                    (Some((p, b)), None)
                } else {
                    (
                        Some((p, b)),
                        Some(Packet {
                            bytes: &self.bytes[cur + 1..self.bytes.len()],
                        }),
                    )
                }
            }
        }
    }

    pub fn new<'b: 'a>(s: &'b str) -> Self {
        Self {
            bytes: s.as_bytes(),
        }
    }
}

impl<'a> Ord for Packet<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.split_first();
        let b = other.split_first();
        match (a.0, b.0) {
            (None, None) => Ordering::Equal,
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (Some((x, bx)), Some((y, by))) => {
                let c = if bx && by {
                    let x_int: usize = std::str::from_utf8(x.bytes).unwrap().parse().unwrap();
                    let y_int: usize = std::str::from_utf8(y.bytes).unwrap().parse().unwrap();
                    x_int.cmp(&y_int)
                } else {
                    x.cmp(&y)
                };
                if c == Ordering::Equal {
                    match (a.1, b.1) {
                        (None, None) => Ordering::Equal,
                        (Some(_), None) => Ordering::Greater,
                        (None, Some(_)) => Ordering::Less,
                        (Some(e), Some(f)) => e.cmp(&f),
                    }
                } else {
                    c
                }
            }
        }
    }
}

impl<'a> PartialOrd for Packet<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Eq for Packet<'a> {}

impl<'a> PartialEq for Packet<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<'a> fmt::Debug for Packet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(std::str::from_utf8(self.bytes).unwrap())
    }
}

mod test {
    #[test]
    fn test() -> Result<(), ()> {
        use super::Packet;
        let p1 = Packet::new("[1,1,3,1,1]");
        let p2 = Packet::new("[1,1,5,1,1]");

        assert_eq!(p1, p1.clone());
        assert!(p1 < p2);
        assert!(p2 > p1);
        assert_eq!(format!("{:?}", p1), "[1,1,3,1,1]");
        let p3 = Packet::new("[[1],[2,3,4]]");
        let p4 = Packet::new("[[1],4]");
        assert!(p3 < p4);
        assert!(p4 > p3);
        let p5 = Packet::new("[9]");
        let p6 = Packet::new("[[8,7,6]]");
        assert!(p5 > p6);
        assert!(p6 < p5);
        let p7 = Packet::new("[[4,4],4,4]");
        let p8 = Packet::new("[[4,4],4,4,4]");
        assert!(p7 < p8);
        assert!(p8 > p7);
        let p9 = Packet::new("[7,7,7,7]");
        let p10 = Packet::new("[7,7,7]");
        assert!(p9 > p10);
        assert!(p10 < p9);
        let p11 = Packet::new("[]");
        let p12 = Packet::new("[3]");
        assert!(p11 < p12);
        assert!(p12 > p11);
        let p13 = Packet::new("[[[]]]");
        let p14 = Packet::new("[[]]");
        assert!(p13 > p14);
        assert!(p14 < p13);
        let p15 = Packet::new("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let p16 = Packet::new("[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert_eq!(format!("{:?}", p16), "[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert!(p15 > p16);
        assert!(p16 < p15);
        Ok(())
    }
}
