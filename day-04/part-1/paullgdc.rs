use std::ops::RangeInclusive;

use aoc::paullgdc::tokenize::Tokenizer;

fn main() {
    aoc::run(run)
}

fn parse_range(range: &mut Tokenizer) -> Option<RangeInclusive<u8>> {
    let start = range.next_u8()?;
    range.eat_byte(b'-')?;
    let end = range.next_u8()?;

    Some(RangeInclusive::new(start, end))
}

fn run(input: &str) -> isize {
    let mut inclusive = 0;
    let mut tokenizer = Tokenizer::new(input.as_bytes());
    (|| -> Option<()> {
        loop {
            let first = parse_range(&mut tokenizer)?;
            tokenizer.eat_byte(b',')?;
            let second = parse_range(&mut tokenizer)?;
            if (first.start() <= second.start() && second.end() <= first.end())
                || (second.start() <= first.start() && first.end() <= second.end())
            {
                inclusive += 1;
            }
            tokenizer.eat_byte(b'\n')?;
        }
    })();
    inclusive
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
