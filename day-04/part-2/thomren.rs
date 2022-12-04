use std::str::FromStr;
use std::error::Error;

fn main() {
    aoc::run(run)
}

fn run(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| AssignementPair::from_str(line).ok())
        .filter(|AssignementPair(a, b)| 
            a.min >= b.min && a.min <= b.max || 
            a.max >= b.min && a.max <= b.max || 
            b.min >= a.min && b.min <= a.max || 
            b.max >= a.min && b.max <= a.max
        )
        .count()
}

struct Assignement {
    min: usize,
    max: usize,
}

impl FromStr for Assignement {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min_str, max_str) = s.split_once("-").ok_or("no '-' delimiter found")?;
        let (min, max) = (min_str.parse()?, max_str.parse()?);
        return Ok(Assignement { min, max })
    }
}

struct AssignementPair(Assignement, Assignement);

impl FromStr for AssignementPair {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_str, second_str) = s.split_once(",").ok_or("no ',' delimiter found")?;
        let (first, second) = (first_str.parse()?, second_str.parse()?);
        return Ok(AssignementPair(first, second))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8".trim()), 4)
    }
}
