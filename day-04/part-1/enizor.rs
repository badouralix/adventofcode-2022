fn main() {
    aoc::run(run)
}

fn run(input: &str) -> isize {
    let mut count = 0;
    // for pair in input.lines() {
    let mut split = input.split(|c| c == ',' || c == '-' || c == '\n');
    while let Some(res) = test_line(&mut split) {
        if res {
            count += 1;
        }
    }
    count
}

fn test_line<'a, I: Iterator<Item = &'a str>>(mut input: I) -> Option<bool> {
    let a = input.next()?.parse::<u32>().ok()?;
    let b = input.next()?.parse::<u32>().ok()?;
    let c = input.next()?.parse::<u32>().ok()?;
    let d = input.next()?.parse::<u32>().ok()?;
    Some((a <= c && b >= d) || (a >= c && b <= d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"),
            2
        )
    }
}
