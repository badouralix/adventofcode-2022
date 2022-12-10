use std::env::args;
use std::ops::RangeInclusive;
use std::time::Instant;

use aoc::paullgdc::tokenize::Tokenizer;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn parse_range(range: &mut Tokenizer) -> Option<RangeInclusive<u8>> {
    let start = range.parse_next_decimal_u8()?;
    range.eat_byte(b'-')?;
    let end = range.parse_next_decimal_u8()?;

    Some(RangeInclusive::new(start, end))
}

fn run(input: &str) -> isize {
    let mut overlap = 0;
    let mut tokenizer = Tokenizer::new(input.as_bytes());
    (|| -> Option<()> {
        loop {
            let first = parse_range(&mut tokenizer)?;
            tokenizer.eat_byte(b',')?;
            let second = parse_range(&mut tokenizer)?;
            if (first.start() <= second.start() && second.start() <= first.end())
                || (second.start() <= first.start() && first.start() <= second.end())
            {
                overlap += 1;
            }
            tokenizer.eat_byte(b'\n')?;
        }
    })();

    overlap
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
            4
        )
    }
}
