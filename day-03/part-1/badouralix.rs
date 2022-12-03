fn main() {
    aoc::run(run)
}

fn run(input: &str) -> isize {
    let mut result = 0;

    for line in input.split('\n') {
        let compartment = line[..line.len() / 2].as_bytes();
        for item in line[line.len() / 2..].as_bytes() {
            if compartment.contains(item) {
                match item {
                    b'a'..=b'z' => result += (*item - b'a' + 1) as isize,
                    b'A'..=b'Z' => result += (*item - b'A' + 27) as isize,
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
