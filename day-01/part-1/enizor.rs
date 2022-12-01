fn main() {
    aoc::run(run)
}

fn run(input: &str) -> u32 {
    let mut max = 0;
    let mut sum = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            max = max.max(sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>().unwrap();
        }
    }
    max.max(sum)
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
