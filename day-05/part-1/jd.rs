use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> String {
    parse(input, 8, 9)
}

fn parse(input: &str, initial_height: usize, stacks_number: usize) -> String {
    let mut stacks: [[u8; 1000]; 9] = [[32; 1000]; 9];
    let mut cursors: [usize; 9] = [0; 9];

    let lines = &mut input.lines();

    for (line_number, line) in lines.enumerate() {
        if line_number == initial_height {
            break;
        }

        let bytes = line.as_bytes();
        for i in 0..stacks_number {
            stacks[i][stacks_number - 1 - line_number] = bytes[1 + i * 4];
            if bytes[1 + i * 4] != 32 && cursors[i] == 0 {
                cursors[i] = stacks_number - line_number;
            }
        }
    }

    lines.next();

    for line in lines {
        let mut words = line.split(' ');
        let count = words.nth(1).unwrap().parse::<usize>().unwrap_or_default();
        let from = words.nth(1).unwrap().parse::<usize>().unwrap_or_default() - 1;
        let to = words.nth(1).unwrap().parse::<usize>().unwrap_or_default() - 1;

        for _ in 0..count {
            let tmp = stacks[from][cursors[from] - 1];
            cursors[from] -= 1;
            stacks[to][cursors[to]] = tmp;
            cursors[to] += 1;
        }
    }

    let mut output = String::with_capacity(stacks_number);
    for i in 0..stacks_number {
        output.push(stacks[i][cursors[i] - 1] as char);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            parse(
                "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
                3,
                3
            ),
            "CMZ"
        )
    }
}
