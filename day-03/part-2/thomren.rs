use std::collections::HashSet;

fn main() {
    aoc::run(run)
}

const GROUP_SIZE: usize = 3;

fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut res = 0;
    for i in 0..(lines.len() / GROUP_SIZE) {
        let group_lines = &lines[(GROUP_SIZE * i)..(GROUP_SIZE * (i + 1))];
        let group_item = find_group_item(group_lines);
        res += priority(group_item);
    }
    res
}

fn find_group_item(lines: &[&str]) -> u8 {
    let common_items = lines
        .into_iter()
        .map(|line| get_unique_chars(*line))
        .reduce(|a, x| a.intersection(&x).cloned().collect())
        .unwrap();
    common_items.into_iter().next().unwrap()
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
CrZsJsPPZsGzwwsLwLmpwMDw".trim()), 70)
    }
}
