use std::env::args;
use std::str::FromStr;
use std::time::Instant;
use std::vec;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

const ROUNDS: usize = 10_000;

fn run(input: &str) -> usize {
    let mut monkeys = input
        .split("\n\n")
        .map(Monkey::from_str)
        .collect::<Result<Vec<Monkey>, _>>()
        .expect("failed to parse monkeys");

    let common_lcm = monkeys.iter().map(|m| m.test as usize).reduce(lcm).unwrap() as isize;

    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                let mut item = monkeys[i].items[j];
                item = (monkeys[i].operation)(item) % common_lcm;
                let target = if item % monkeys[i].test == 0 {
                    monkeys[i].true_target
                } else {
                    monkeys[i].false_target
                };
                monkeys[target].items.push(item);
            }
            monkeys[i].inspections_counter += monkeys[i].items.len();
            monkeys[i].items = vec![];
        }
    }

    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.inspections_counter).collect();
    inspections.sort();
    inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = (a.max(b), a.min(b));
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

struct Monkey {
    items: Vec<isize>,
    operation: Box<dyn Fn(isize) -> isize>,
    test: isize,
    true_target: usize,
    false_target: usize,
    inspections_counter: usize,
}

impl FromStr for Monkey {
    type Err = Box<dyn std::error::Error>;

    #[allow(clippy::needless_late_init)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();

        let (_, starting_items) = lines[1]
            .split_once(": ")
            .ok_or("cannot parse starting items")?;
        let items = starting_items
            .split(", ")
            .map(|x| x.parse())
            .collect::<Result<Vec<isize>, _>>()?;

        let op = lines[2].as_bytes()[23];
        let operand = &lines[2][25..].parse::<isize>();
        let operation: Box<dyn Fn(isize) -> isize>;
        match (op, operand) {
            (b'*', Err(_)) => operation = Box::new(|x| x * x),
            (b'*', &Ok(n)) => operation = Box::new(move |x| x * n),
            (b'+', Err(_)) => operation = Box::new(|x| x + x),
            (b'+', &Ok(n)) => operation = Box::new(move |x| x + n),
            (b'-', Err(_)) => operation = Box::new(|_| 0),
            (b'-', &Ok(n)) => operation = Box::new(move |x| x - n),
            (b'/', Err(_)) => operation = Box::new(|_| 1),
            (b'/', &Ok(n)) => operation = Box::new(move |x| x / n),
            _ => {
                return Err(Box::from(format!("cannot parse operation: {}", op as char)));
            }
        };

        let test = lines[3][21..].parse()?;

        let true_target = lines[4][29..].parse::<usize>()?;
        let false_target = lines[5][30..].parse::<usize>()?;

        Ok(Monkey {
            items,
            operation,
            test,
            true_target,
            false_target,
            inspections_counter: 0,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"),
            2713310158
        )
    }

    #[test]
    fn test_monkey_parsing() {
        let monkey = Monkey::from_str(
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3",
        )
        .unwrap();
        assert_eq!(monkey.items, vec![79, 98]);
        assert_eq!((monkey.operation)(5), 5 * 19);
        assert_eq!(monkey.test, 23);
        assert_eq!(monkey.true_target, 2);
        assert_eq!(monkey.false_target, 3);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(1, 1), 1);
        assert_eq!(lcm(6, 9), 18);
        assert_eq!(lcm(9, 6), 18);
        assert_eq!(lcm(34, 45), 1530);
    }
}
