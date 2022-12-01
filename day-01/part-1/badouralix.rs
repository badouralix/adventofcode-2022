use std::cmp;

fn main() {
    aoc::run(run);
}

fn run(input: &str) -> isize {
    let split = input.split('\n');
    let mut max = 0;
    let mut sum = 0;

    for line in split {
        if line.is_empty() {
            max = cmp::max(max, sum);
            sum = 0;
            continue;
        }

        sum += line.parse::<isize>().unwrap();
    }
    max = cmp::max(max, sum);

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"),
            24000
        )
    }
}
