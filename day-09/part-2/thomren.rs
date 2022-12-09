use core::panic;
use std::{collections::HashSet, slice::Iter, str::FromStr};

fn main() {
    aoc::run(run)
}

const ROPE_LENGTH: usize = 10;

fn run(input: &str) -> usize {
    let mut tail_seen = HashSet::<Point>::new();
    let mut rope = vec![Point(0, 0); ROPE_LENGTH];
    tail_seen.insert(rope[0]);
    for line in input.lines() {
        let ins = Instruction::from_str(line).unwrap();

        for _ in 0..ins.n_steps {
            let head = rope.last_mut().unwrap();
            head.0 += ins.direction.0;
            head.1 += ins.direction.1;
            for i in (0..(rope.len() - 1)).rev() {
                rope[i] = follow(rope[i], rope[i + 1]);
            }
            tail_seen.insert(rope[0]);
        }
    }

    tail_seen.len()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point(isize, isize);

fn follow(p: Point, head: Point) -> Point {
    let dx: isize = head.0 - p.0;
    let dy: isize = head.1 - p.1;
    match (dx, dy) {
        (dx, dy) if (dx.abs() + dy.abs()) <= 1 => p,
        (0, dy) => Point(p.0, p.1 + dy.signum()),
        (dx, 0) => Point(p.0 + dx.signum(), p.1),
        (dx, dy) if (dx.abs() + dy.abs()) > 2 => Point(p.0 + dx.signum(), p.1 + dy.signum()),
        _ => p,
    }
}

struct Instruction {
    direction: Point,
    n_steps: usize,
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();
        let (dx, dy) = match s[0] {
            b'R' => (1, 0),
            b'U' => (0, 1),
            b'L' => (-1, 0),
            b'D' => (0, -1),
            _ => panic!("invalid direction: {}", s[0] as char),
        };
        let n_steps = atoi(&mut s[2..].iter());
        Ok(Self {
            direction: Point(dx, dy),
            n_steps,
        })
    }
}

/// Parse a number from a bytes iterator, stopping when a
/// non-digit character is encountered
fn atoi(it: &mut Iter<u8>) -> usize {
    let mut res = 0;
    for &b in it {
        match b {
            b'0'..=b'9' => {}
            _ => break,
        }
        res *= 10;
        res += (b - b'0') as usize;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
            .trim()),
            1
        );

        assert_eq!(
            run("
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
                .trim()),
            36
        );
    }
}
