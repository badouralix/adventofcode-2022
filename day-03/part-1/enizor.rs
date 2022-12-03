use aoc::enizor::BitSet;
fn main() {
    aoc::run(run)
}

fn run(input: &str) -> u32 {
    // Your code goes here
    let bytes = input.as_bytes();
    let mut res = 0;
    for line in bytes.split(|&b| b == b'\n') {
        let mut left = BitSet::<1>::default();
        let mut right = BitSet::<1>::default();
        let l = line.len();
        for i in 0..(l / 2) {
            left.set((line[i] - b'A') as usize);
            right.set((line[i + (l / 2)] - b'A') as usize);
        }
        let shared = (left & right).leading_zeros();
        // dbg!((shared as u8 + b'A') as char);
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
