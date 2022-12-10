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
    // Your code goes here
    let mut crates: [String; 9] = Default::default();
    let mut split = input.split("\n\n");

    for line in split.next().unwrap().split('\n').rev().skip(1) {
        for (i, item) in crates.iter_mut().enumerate() {
            if line.len() > 4 * i && line.as_bytes()[4 * i + 1] != b' ' {
                item.push(line.as_bytes()[4 * i + 1] as char);
            }
        }
    }

    for line in split.next().unwrap().split('\n') {
        let mut step = line.split(' ');
        step.next();
        let size = step.next().unwrap().parse::<usize>().unwrap();
        step.next();
        let from = (step.next().unwrap().as_bytes()[0] - b'1') as usize;
        step.next();
        let to = (step.next().unwrap().as_bytes()[0] - b'1') as usize;

        crates[to] = format!(
            "{}{}",
            crates[to].clone(),
            &crates[from][crates[from].len() - size..]
        );
        crates[from].truncate(crates[from].len() - size);
    }

    crates
        .iter()
        .fold(String::new(), |acc, item| match item.chars().last() {
            None => acc,
            Some(c) => format!("{}{}", acc, c),
        })
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
move 1 from 1 to 2"),
            "MCD"
        )
    }
}
