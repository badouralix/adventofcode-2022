use std::slice::Iter;
use std::str::FromStr;
use std::error::Error;

fn main() {
    aoc::run(run)
}

fn run(input: &str) -> usize {
    input
        .lines()
        .map(|line| AssignementPair::from_str(line).unwrap())
        .filter(|AssignementPair(a, b)| 
            b.0 <= a.0 && a.0 <= b.1 || 
            a.0 <= b.0 && b.0 <= a.1
        )
        .count()
}

struct Assignement(usize, usize);

struct AssignementPair(Assignement, Assignement);

impl FromStr for AssignementPair {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.as_bytes().into_iter();
        let first_min = atoi(it.by_ref());
        let first_max = atoi(it.by_ref());
        let second_min= atoi(it.by_ref());
        let second_max = atoi(it.by_ref());

        let res = AssignementPair(
            Assignement(first_min, first_max), 
            Assignement(second_min, second_max)
        );
        return Ok(res)
    }
}

/// Parse a number from a bytes iterator, stopping when a
/// non-digit character is encountered
fn atoi(it: &mut Iter<u8>) -> usize {
    let mut res = 0;
    for &b in it {
        match b {
            b'0'..=b'9' => {},
            _ => break,
        }
        res *= 10;
        res += (b - b'0') as usize;
    }
    res
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
