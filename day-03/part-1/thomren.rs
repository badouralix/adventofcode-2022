use std::collections::HashSet;

fn main() {
    aoc::run(run)
}

fn run(input: &str) -> usize {
    input.lines().map(find_common_item).map(priority).sum()
}

fn find_common_item(line: &str) -> u8 {
    let first_compartment = &line[..line.len() / 2];
    let second_compartment = &line[(line.len() / 2)..];
    let first_compartment_items = get_unique_chars(first_compartment);
    let second_compartment_items = get_unique_chars(second_compartment);
    let intersection: Vec<&u8> = first_compartment_items.intersection(&second_compartment_items).collect();
    return *intersection.into_iter().next().unwrap()
}

fn get_unique_chars(line: &str) -> HashSet<u8> {
    line.as_bytes().into_iter().map(|x| *x).collect::<HashSet<u8>>()
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
        assert_eq!(run("
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".trim()), 157)
    }
}
