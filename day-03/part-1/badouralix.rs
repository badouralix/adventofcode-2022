use std::collections::HashSet;

fn main() {
    aoc::run(run)
}

fn run(input: &str) -> isize {
    let mut result = 0;

    for line in input.split('\n') {
        let compartment: HashSet<char> = HashSet::from_iter(line[..line.len() / 2].chars());
        for item in line[line.len() / 2..].chars() {
            if compartment.contains(&item) {
                match item {
                    'a'..='z' => result += item as isize - 'a' as isize + 1,
                    'A'..='Z' => result += item as isize - 'A' as isize + 27,
                    _ => todo!(),
                }
                break;
            }
        }
    }

    result
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
