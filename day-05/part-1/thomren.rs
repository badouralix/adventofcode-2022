use std::str::{FromStr, Utf8Error};

fn main() {
    aoc::run(run)
}

fn run(input: &str) -> String {
    let (starting_stacks, procedure) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(starting_stacks);
    let instructions = procedure
        .lines()
        .filter_map(|s| Instruction::from_str(s).ok());

    for ins in instructions {
        for _ in 1..=ins.n {
            let x = stacks[ins.from].pop().unwrap();
            stacks[ins.to].push(x);
        }
    }

    get_stack_tops_as_str(&stacks).unwrap()
}

fn parse_stacks(s: &str) -> Vec<Vec<u8>> {
    let stack_lines = s.lines().collect::<Vec<&str>>();
    let nb_stacks = (stack_lines[stack_lines.len() - 1].len() + 2) / 4;
    let mut stacks = vec![Vec::new(); nb_stacks];
    for line in stack_lines.into_iter().rev().skip(1) {
        let bytes = line.as_bytes();
        for (k, i) in (1..(4 * stacks.len())).step_by(4).enumerate() {
            match bytes.get(i) {
                Some(&b) if b != b' ' => stacks[k].push(b),
                _ => {}
            }
        }
    }
    stacks
}

struct Instruction {
    n: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(' ').collect::<Vec<&str>>();
        let n = tokens[1].parse::<usize>().unwrap();
        let from = tokens[3].parse::<usize>().unwrap() - 1;
        let to = tokens[5].parse::<usize>().unwrap() - 1;
        Ok(Self { n, from, to })
    }
}

fn get_stack_tops_as_str(stacks: &[Vec<u8>]) -> Result<String, Utf8Error> {
    let res: Vec<u8> = stacks.iter().filter_map(|s| s.last().copied()).collect();
    Ok(std::str::from_utf8(&res)?.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
                .trim_start_matches('\n')),
            "CMZ"
        )
    }

    #[test]
    fn test_parse_stacks() {
        assert_eq!(
            parse_stacks(
                "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3 "
                    .trim_start_matches('\n')
            ),
            vec![vec![b'Z', b'N'], vec![b'M', b'C', b'D'], vec![b'P']]
        )
    }
}
