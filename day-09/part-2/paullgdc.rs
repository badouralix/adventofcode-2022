use std::collections::HashSet;

use aoc::paullgdc::tokenize::Tokenizer;

fn main() {
    aoc::run(run)
}

#[derive(Debug)]
struct Rope {
    knots: [(i32, i32); Self::KNOTS],
}

impl Rope {
    const KNOTS: usize = 10;

    fn tail(&self) -> (i32, i32) {
        *self.knots.last().unwrap()
    }

    fn propagate_move(&mut self) {
        for i in 0..(Self::KNOTS - 1) {
            self.follow_knot(i);
        }
    }

    fn follow_knot(&mut self, knot: usize) {
        if !((self.knots[knot].1 - 1..=self.knots[knot].1 + 1).contains(&self.knots[knot + 1].1)
            && (self.knots[knot].0 - 1..=self.knots[knot].0 + 1).contains(&self.knots[knot + 1].0))
        {
            self.knots[knot + 1].1 += (self.knots[knot].1 - self.knots[knot + 1].1).signum();
            self.knots[knot + 1].0 += (self.knots[knot].0 - self.knots[knot + 1].0).signum();
        }
    }

    fn apply_instruction(&mut self, dir: u8) {
        match dir {
            b'R' => self.knots[0].0 += 1,
            b'L' => self.knots[0].0 -= 1,
            b'U' => self.knots[0].1 += 1,
            b'D' => self.knots[0].1 -= 1,
            _ => {}
        }
    }
}

fn run(input: &str) -> usize {
    let mut tokenizer = Tokenizer::new(input.as_bytes());
    let mut tail_positions = HashSet::new();
    let mut rope = Rope {
        knots: [(0, 0); Rope::KNOTS],
    };
    tail_positions.insert(rope.tail());
    while !tokenizer.end() {
        let dir = tokenizer.next_ascii_char().unwrap();
        tokenizer.eat_byte(b' ').unwrap();
        let dist = tokenizer.parse_next_decimal_u8().unwrap();
        for _ in 0..dist {
            rope.apply_instruction(dir);
            rope.propagate_move();
            tail_positions.insert(rope.tail());
        }
        tokenizer.eat_byte(b'\n');
    }
    tail_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"),
            36
        )
    }
}
