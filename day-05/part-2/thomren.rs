fn main() {
    aoc::run(run)
}

fn run(input: &str) -> String {
    let (starting_stacks, procedure) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(starting_stacks);
    for instruction in procedure.lines() {
        let words = instruction.split(' ').collect::<Vec<&str>>();
        let n = words[1].parse::<usize>().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let to = words[5].parse::<usize>().unwrap() - 1;
        for i in (stacks[from].len() - n)..(stacks[from].len()) {
            let x = stacks[from][i];
            stacks[to].push(x);
        }
        let from_size = stacks[from].len() - n;
        stacks[from].truncate(from_size);
    }
    let res: Vec<u8> = stacks.iter().filter_map(|s| s.last().copied()).collect();
    std::str::from_utf8(&res).unwrap().to_string()
}

fn parse_stacks(s: &str) -> Vec<Vec<u8>> {
    let stack_lines = s.lines().collect::<Vec<&str>>();
    let nb_stacks = (stack_lines[stack_lines.len()-1].len() + 1) / 4;
    let max_height = nb_stacks * (stack_lines.len() - 1);
    let mut stacks = vec![Vec::with_capacity(max_height); nb_stacks];
    for line in stack_lines.into_iter().rev().skip(1) {
        let bytes = line.as_bytes();
        for (k, i) in (1..bytes.len()).step_by(4).enumerate() {
            if bytes[i] != b' ' {
                stacks[k].push(bytes[i]);
            }
        }
    }
    stacks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2".trim_start_matches('\n')), "CMZ")
    }

    #[test]
    fn test_parse_stacks() {
        assert_eq!(parse_stacks("
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ".trim_start_matches('\n')
        ), vec![vec![b'Z', b'N'], vec![b'M', b'C', b'D'], vec![b'P']]
    )
    }
}
