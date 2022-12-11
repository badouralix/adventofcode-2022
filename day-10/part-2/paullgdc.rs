use std::env::args;
use std::time::Instant;

use aoc::paullgdc::tokenize::Tokenizer;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("_parse");

    println!("{}", output);
}

fn run(input: &str) -> String {
    let mut tokenizer = Tokenizer::new(input.as_bytes());
    let instructions = parse_instructions(&mut tokenizer).unwrap();
    let mut clock = Clock {
        cycle: 0,
        x_register: 1,
    };
    let mut render = String::with_capacity(41 * 6);
    for instr in instructions {
        match instr {
            Instruction::Noop => {
                if ((clock.x_register - 1)..=clock.x_register + 1).contains(&clock.cycle) {
                    render.push('#');
                } else {
                    render.push('.');
                }
                clock.cycle += 1;
                if clock.cycle % 40 == 0 {
                    clock.cycle -= 40;
                    render.push('\n');
                }
            }
            Instruction::AddX(addx) => {
                for _ in 0..2 {
                    if ((clock.x_register - 1)..=clock.x_register + 1).contains(&clock.cycle) {
                        render.push('#');
                    } else {
                        render.push('.');
                    }
                    clock.cycle += 1;
                    if clock.cycle % 40 == 0 {
                        clock.cycle -= 40;
                        render.push('\n');
                    }
                }
                clock.x_register += addx
            }
        }
    }
    render.pop();
    render
}

#[derive(Debug)]
struct Clock {
    cycle: i32,
    x_register: i32,
}

#[derive(Debug)]
enum Instruction {
    AddX(i32),
    Noop,
}

fn parse_instructions(tokenizer: &mut Tokenizer) -> Option<Vec<Instruction>> {
    let mut instructions = Vec::new();
    use Instruction::*;
    while !tokenizer.end() {
        instructions.push(match tokenizer.consume_next_token() {
            Some(b"noop") => Noop,
            Some(b"addx") => {
                tokenizer.eat_byte(b' ');
                AddX(tokenizer.parse_next_decimal_i32()?)
            }
            _ => return dbg!(None),
        });
        tokenizer.eat_byte_or_end(b'\n')?;
    }
    Some(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("addx 15
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
noop"),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        )
    }
}
