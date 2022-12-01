fn main() {
    aoc::run(run)
}

fn run(input: &str) -> isize {
    let split = input.split('\n');
    let mut sum = 0;
    let mut sums = Vec::new();

    for line in split {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
            continue;
        }

        sum += line.parse::<isize>().unwrap();
    }
    sums.push(sum);

    sums.sort();
    sums.reverse();

    sums[0] + sums[1] + sums[2]
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
            45000
        )
    }
}
