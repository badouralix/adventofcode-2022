use aoc::enizor::BitSet;

fn main() {
    aoc::run(run)
}

type Compartment = BitSet<1>;

fn run(input: &str) -> u32 {
    // Your code goes here
    let bytes = input.as_bytes();
    let mut res = 0;
    for line in bytes.split(|&b| b == b'\n') {
        let mut left = Compartment::default();
        let mut right = Compartment::default();
        let l = line.len();
        for c in &line[..l / 2] {
            left.set(c - b'A');
        }
        for c in &line[l / 2..] {
            right.set(c - b'A');
        }
        let shared = (left & right).first_set();
        res += priority(shared);
    }
    res
}

fn priority(n: u32) -> u32 {
    if n < 26 {
        27 + n
    } else {
        n - (b'a' - b'A' - 1) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(priority((b'A' - b'A') as u32), 27);
        assert_eq!(priority((b'Z' - b'A') as u32), 52);
        assert_eq!(priority((b'a' - b'A') as u32), 1);
        assert_eq!(priority((b'z' - b'A') as u32), 26);
        assert_eq!(
            run("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"),
            157
        )
    }
}
