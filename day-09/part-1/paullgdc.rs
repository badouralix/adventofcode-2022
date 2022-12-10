use std::collections::HashSet;
use std::env::args;
use std::time::Instant;

use aoc::paullgdc::tokenize::Tokenizer;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

#[derive(Debug)]
struct Rope {
    knots: [(i32, i32); Self::KNOTS],
}

impl Rope {
    const KNOTS: usize = 2;

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
            run("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"),
            13
        )
    }
}
