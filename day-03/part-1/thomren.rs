fn main() {
    aoc::run(run)
}

fn run(input: &str) -> usize {
    input.lines().map(find_common_item).map(priority).sum()
}

fn find_common_item(line: &str) -> u8 {
    let mut seen = [false; 256];
    // first compartment
    for &b in line[..(line.len() / 2)].as_bytes() {
        seen[b as usize] = true;
    }
    // second compartment
    for &b in line[(line.len() / 2)..].as_bytes() {
        if seen[b as usize] {
            return b;
        }
    }
    0
}

fn priority(c: u8) -> usize {
    match c {
        b'A'..=b'Z' => (c - b'A') as usize + 27,
        b'a'..=b'z' => (c - b'a') as usize + 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
                .trim()),
            157
        )
    }
}
