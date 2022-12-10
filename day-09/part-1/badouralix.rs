use std::env::args;
use std::time::Instant;
use std::{cmp, collections::HashSet};

struct Rope {
    head: (isize, isize),
    tail: (isize, isize),
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: (0, 0),
            tail: (0, 0),
        }
    }

    fn len(&self) -> usize {
        cmp::max(
            (self.head.0 - self.tail.0).abs(),
            (self.head.1 - self.tail.1).abs(),
        )
        .try_into()
        .unwrap()
    }

    fn move_head(&mut self, direction: char) {
        match direction {
            'D' => self.head = (self.head.0, self.head.1 - 1),
            'L' => self.head = (self.head.0 - 1, self.head.1),
            'R' => self.head = (self.head.0 + 1, self.head.1),
            'U' => self.head = (self.head.0, self.head.1 + 1),
            _ => unreachable!(),
        }

        self.move_tail()
    }

    fn move_tail(&mut self) {
        if self.len() <= 1 {
            return;
        }

        if self.head.0 - self.tail.0 < -1 {
            self.tail = (self.head.0 + 1, self.head.1)
        }

        if self.head.0 - self.tail.0 > 1 {
            self.tail = (self.head.0 - 1, self.head.1)
        }

        if self.head.1 - self.tail.1 < -1 {
            self.tail = (self.head.0, self.head.1 + 1)
        }

        if self.head.1 - self.tail.1 > 1 {
            self.tail = (self.head.0, self.head.1 - 1)
        }
    }
}

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> usize {
    let mut history = HashSet::<(isize, isize)>::new();
    let mut rope = Rope::new();

    for line in input.split('\n') {
        let direction = line.chars().next().unwrap();
        let steps: usize = line.chars().skip(2).collect::<String>().parse().unwrap();

        for _ in 0..steps {
            rope.move_head(direction);
            history.insert(rope.tail);
        }
    }

    history.len()
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
            13
        )
    }
}
