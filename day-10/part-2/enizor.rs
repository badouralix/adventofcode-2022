use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("_parse");
    println!("{}", output);
}

#[derive(Debug)]
pub struct CPU {
    x: isize,
    clock: isize,
    screen: String,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            x: 1,
            clock: 0,
            screen: String::new(),
        }
    }
}

enum Instruction {
    Noop,
    AddX(isize),
}

use Instruction::*;

impl CPU {
    fn run_clock(&mut self) {
        if (self.x - self.clock).abs() <= 1 {
            self.screen.push('#');
        } else {
            self.screen.push('.');
        }
        self.clock += 1;
        if self.clock == 40 {
            self.clock = 0;
            self.screen.push('\n');
        }
    }

    fn run(&mut self, op: Instruction) {
        match op {
            Noop => self.run_clock(),
            AddX(v) => {
                self.run_clock();
                self.run_clock();
                self.x += v;
            }
        }
    }
}

fn run(input: &str) -> String {
    let mut cpu = CPU::default();
    let mut words = input.trim().split_ascii_whitespace();
    while let Some(w) = words.next() {
        let op = match w {
            "noop" => Noop,
            "addx" => AddX(words.next().unwrap().parse().unwrap()),
            _ => panic!(),
        };
        cpu.run(op);
    }
    cpu.screen.push('\n');
    cpu.screen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let e1 = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1";
        assert_eq!(run(e1), "##..##..##..##..##..#");
        let example = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        assert_eq!(
            run(example),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }
}
