use aoc::enizor::BitSet;

fn main() {
    aoc::run(run)
}

type RuckSack = BitSet<1>;

fn run(input: &str) -> u32 {
    // Your code goes here
    let bytes = input.as_bytes();
    let mut res = 0;
    let mut group_count = 0;
    let mut group = RuckSack::ones();
    let mut current_elve = RuckSack::default();
    for (i, &c) in bytes.iter().enumerate() {
        if c != b'\n' {
            current_elve.set(c - b'A');
        }
        if c == b'\n' || i == bytes.len() - 1 {
            group_count = (group_count + 1) % 3;
            group &= current_elve;
            current_elve = RuckSack::default();
            if group_count == 0 {
                let badge = group.first_set();
                group = RuckSack::ones();
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
