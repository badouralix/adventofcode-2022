use std::collections::HashSet;

fn main() {
    aoc::run(run)
}

fn run(input: &str) -> isize {
    let mut result = 0;
    let mut group: Vec<HashSet<char>> = Vec::new();

    for line in input.split('\n') {
        group.push(HashSet::from_iter(line.chars()));

        if group.len() < 3 {
            continue;
        }

        for item in &group[0] {
            if group[1].contains(item) && group[2].contains(item) {
                match item {
                    'a'..='z' => result += *item as isize - 'a' as isize + 1,
                    'A'..='Z' => result += *item as isize - 'A' as isize + 27,
                    _ => todo!(),
                }
                break;
            }
        }

        group = Vec::new();
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
            70
        )
    }
}
