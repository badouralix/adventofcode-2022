use aoc::enizor::BitSet;
fn main() {
    aoc::run(run)
}

fn run(input: &str) -> u32 {
    // Your code goes here
    let bytes = input.as_bytes();
    let mut res = 0;
    let mut group = [BitSet::<1>::default(); 3];
    let mut group_count = 0;
    for (i, &c) in bytes.iter().enumerate() {
        if c != b'\n' {
            group[group_count].set((c - b'A') as usize);
        }
        if c == b'\n' || i == bytes.len() - 1 {
            group_count = (group_count + 1) % 3;
            if group_count == 0 {
                let badge = (group[0] & group[1] & group[2]).leading_zeros();
                group = [BitSet::<1>::default(); 3];

                res += priority(badge);
            }
        }
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
        assert_eq!(
            run("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"),
            70
        )
    }
}
