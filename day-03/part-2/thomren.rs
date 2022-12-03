
fn main() {
    aoc::run(run)
}

const GROUP_SIZE: usize = 3;

fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    assert!(lines.len() % GROUP_SIZE == 0);

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
        .map(|&line| get_bytes_set(line))
        .reduce(|a, x| a & x)
        .unwrap();
    ilog(common_items) as u8
}

/// Create a set of bytes represented by a 128-bit mask from a string.
/// Only characters up to \u{80} are supported.
fn get_bytes_set(line: &str) -> u128 {
    let mut res = 0;
    for &b in line.as_bytes() {
        res |= 1 << b;
    }
    res
}

/// Base 2 logarithm of a u128
fn ilog(x: u128) -> u32 {
  x.next_power_of_two().trailing_zeros()
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
