use core::fmt;
use std::{cmp, collections::HashSet};

const SIZE: usize = 10;

struct Rope {
    knots: [(isize, isize); SIZE],
}

impl Rope {
    fn new() -> Rope {
        Rope {
            knots: [(0, 0); 10],
        }
    }

    fn move_head(&mut self, direction: char) {
        match direction {
            'D' => self.knots[0] = (self.knots[0].0, self.knots[0].1 - 1),
            'L' => self.knots[0] = (self.knots[0].0 - 1, self.knots[0].1),
            'R' => self.knots[0] = (self.knots[0].0 + 1, self.knots[0].1),
            'U' => self.knots[0] = (self.knots[0].0, self.knots[0].1 + 1),
            _ => unreachable!(),
        }

        for idx in 1..SIZE {
            self.move_knot(idx)
        }
    }

    fn move_knot(&mut self, idx: usize) {
        if cmp::max(
            (self.knots[idx - 1].0 - self.knots[idx].0).abs(),
            (self.knots[idx - 1].1 - self.knots[idx].1).abs(),
        ) <= 1
        {
            return;
        }

        let x = if self.knots[idx - 1].0 - self.knots[idx].0 < -1 {
            self.knots[idx - 1].0 + 1
        } else if self.knots[idx - 1].0 - self.knots[idx].0 > 1 {
            self.knots[idx - 1].0 - 1
        } else {
            self.knots[idx - 1].0
        };

        let y = if self.knots[idx - 1].1 - self.knots[idx].1 < -1 {
            self.knots[idx - 1].1 + 1
        } else if self.knots[idx - 1].1 - self.knots[idx].1 > 1 {
            self.knots[idx - 1].1 - 1
        } else {
            self.knots[idx - 1].1
        };

        self.knots[idx] = (x, y);
    }

    fn tail(&self) -> (isize, isize) {
        self.knots[SIZE - 1]
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in (0..5).rev() {
            for x in 0..6 {
                let mut flag = true;

                for k in 0..SIZE {
                    if self.knots[k].0 == x && self.knots[k].1 == y {
                        flag = false;
                        if k == 0 {
                            write!(f, "H")?;
                        } else {
                            write!(f, "{}", k)?;
                        }
                        break;
                    }
                }

                if flag {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    aoc::run(run)
}

fn run(input: &str) -> usize {
    let mut history = HashSet::<(isize, isize)>::new();
    let mut rope = Rope::new();

    for line in input.split('\n') {
        let direction = line.chars().next().unwrap();
        let steps: usize = line.chars().skip(2).collect::<String>().parse().unwrap();

        for _ in 0..steps {
            rope.move_head(direction);
            history.insert(rope.tail());
        }
    }

    history.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_example() {
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
        )
    }

    #[test]
    fn large_example() {
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
        )
    }
}
