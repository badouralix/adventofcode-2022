use std::hint::unreachable_unchecked;

fn main() {
    aoc::run(run)
}

struct RuckSack(u64);

impl RuckSack {
    unsafe fn from_items(items: &[u8]) -> Self {
        let mut s = Self(0);
        for item in items {
            s.add(*item)
        }
        s
    }

    unsafe fn add(&mut self, item: u8) {
        match item {
            b'a'..=b'z' => self.0 |= 1 << item - b'a',
            b'A'..=b'Z' => self.0 |= 1 << (item - b'A' + 26),
            _ => unreachable_unchecked(),
        }
    }

    fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    fn points(&self) -> u32 {
        self.0.trailing_zeros() + 1
    }
}

fn run(input: &str) -> u32 {
    let mut priority = 0;
    for line in input.split('\n') {
        let line = line.as_bytes();
        
        unsafe {
            let left = RuckSack::from_items(&line[..line.len() / 2]);
            let right = RuckSack::from_items(&line[line.len() / 2..]);
            priority += left.intersection(right).points()
        }
    }
    priority
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"), 157)
    }
}
