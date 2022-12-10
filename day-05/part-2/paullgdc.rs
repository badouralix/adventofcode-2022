use aoc::paullgdc::{get_mut_2, tokenize::Tokenizer};
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

const N: usize = 9;

fn parse_stack(tokenizer: &mut Tokenizer) -> [Vec<u8>; N] {
    const EMPTY_VEC: Vec<u8> = Vec::new();
    let mut stacks = [EMPTY_VEC; N];
    'parse_stack: loop {
        for stack in stacks.iter_mut() {
            if tokenizer
                .next_nth_byte(1)
                .map(|b| u8::is_ascii_digit(&b))
                .unwrap_or(true)
            {
                break 'parse_stack;
            }
            if tokenizer.next_nth_byte(0) == Some(b'[') {
                tokenizer.eat_byte(b'[');
                stack.push(tokenizer.next_ascii_char().unwrap());
                tokenizer.eat_byte(b']');
            } else {
                tokenizer.advance(3);
            }
            if tokenizer.next_nth_byte(0) == Some(b'\n') {
                tokenizer.advance(1);
                break;
            } else {
                tokenizer.advance(1);
            }
        }
    }

    while tokenizer
        .next_nth_byte(0)
        .map(|b| b != b'\n')
        .unwrap_or(false)
    {
        tokenizer.advance(1);
    }

    tokenizer.advance(1);
    for stack in &mut stacks {
        stack.reverse();
    }
    stacks
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    from: u8,
    to: u8,
    quantity: u8,
}

fn parse_move_instructions(tokenizer: &mut Tokenizer) -> Vec<Instruction> {
    let mut instructions = Vec::with_capacity(100);
    loop {
        tokenizer.eat_chars(b"move ");
        let quantity = tokenizer.parse_next_decimal_u8().unwrap();
        tokenizer.eat_chars(b" from ");
        let from = tokenizer.parse_next_decimal_u8().unwrap() - 1;
        tokenizer.eat_chars(b" to ");
        let to = tokenizer.parse_next_decimal_u8().unwrap() - 1;
        instructions.push(Instruction { from, to, quantity });
        if tokenizer.eat_byte(b'\n').is_none() {
            break;
        }
    }
    instructions
}

fn process_instruction(stacks: &mut [Vec<u8>; N], instruction: Instruction) {
    let (from, to) = get_mut_2(stacks, instruction.from as usize, instruction.to as usize).unwrap();
    to.extend(from.drain(from.len() - instruction.quantity as usize..));
}

fn run(input: &str) -> String {
    let mut tokenizer = Tokenizer::new(input.as_bytes());
    let mut stacks = parse_stack(&mut tokenizer);
    tokenizer.eat_byte(b'\n').unwrap();
    let instructions = parse_move_instructions(&mut tokenizer);
    for instruction in instructions {
        process_instruction(&mut stacks, instruction);
    }
    String::from_utf8(
        stacks
            .into_iter()
            .take_while(|s| !s.is_empty())
            .map(|s| *s.last().unwrap())
            .collect(),
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"),
            "MCD"
        )
    }
}
