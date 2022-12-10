use std::env::args;
use std::str::{FromStr, Split};
use std::time::Instant;

#[allow(clippy::upper_case_acronyms)]
struct CPU<'a> {
    cycle: usize,
    instructions: Split<'a, char>,
    next: (usize, Option<isize>),
    register_x: isize,
}

impl CPU<'_> {
    fn new(instructions: Split<char>) -> CPU {
        CPU {
            cycle: 0,
            instructions,
            next: (0, None),
            register_x: 1,
        }
    }

    fn run(&mut self) {
        self.cycle += 1;

        if self.cycle < self.next.0 {
            return;
        }

        if let Some(value) = self.next.1 {
            self.register_x += value;
        }

        match self.instructions.next() {
            Some(instruction) if instruction == "noop" => self.next = (self.cycle + 1, None),
            Some(instruction) if instruction.starts_with("addx") => {
                let value: isize = instruction
                    .chars()
                    .skip(5)
                    .collect::<String>()
                    .parse()
                    .unwrap();
                self.next = (self.cycle + 2, Some(value))
            }
            Some(_) => unimplemented!(),
            None => {}
        }
    }
}

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("_parse");
    println!("{}", output);
}

fn run(input: &str) -> String {
    let instructions = input.split('\n');
    let mut cpu = CPU::new(instructions);
    let mut result = String::from_str("").unwrap();

    for pixel in 0..240 {
        cpu.run();

        if (pixel % 40 - cpu.register_x).abs() <= 1 {
            result.push('#');
        } else {
            result.push('.');
        }

        if cpu.cycle % 40 == 0 {
            result.push('\n');
        }
    }

    result.trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("
addx 15
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
noop"
                .trim()),
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .trim()
        )
    }
}
