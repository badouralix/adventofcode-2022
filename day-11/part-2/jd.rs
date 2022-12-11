use std::collections::VecDeque;
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

#[derive(Debug, Clone, Default)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: usize,
    true_outcome: usize,
    false_outcome: usize,
    count: usize,
}

#[derive(Debug, Clone)]
enum Operation {
    Addition(usize),
    Multiplication(usize),
    Square,
}

impl Default for Operation {
    fn default() -> Self {
        Self::Addition(0)
    }
}

fn run(input: &str) -> usize {
    let mut monkeys = Vec::with_capacity(7);

    let mut current_monkey = Monkey::default();

    for line in input.lines() {
        if line.is_empty() {
            monkeys.push(current_monkey.clone());
            current_monkey = Monkey::default();
        } else if let Some(items) = line.strip_prefix("  Starting items: ") {
            current_monkey.items = items
                .split(", ")
                .map(|i| i.parse::<usize>().unwrap_or_default())
                .collect::<VecDeque<usize>>();
        } else if let Some(operation) = line.strip_prefix("  Operation: new = old ") {
            if let Some(operand) = operation.strip_prefix("* ") {
                if operand == "old" {
                    current_monkey.operation = Operation::Square;
                } else {
                    current_monkey.operation =
                        Operation::Multiplication(operand.parse::<usize>().unwrap_or_default());
                }
            } else if let Some(operand) = operation.strip_prefix("+ ") {
                current_monkey.operation =
                    Operation::Addition(operand.parse::<usize>().unwrap_or_default());
            }
        } else if let Some(test) = line.strip_prefix("  Test: divisible by ") {
            current_monkey.test = test.parse::<usize>().unwrap_or_default();
        } else if let Some(true_outcome) = line.strip_prefix("    If true: throw to monkey ") {
            current_monkey.true_outcome = true_outcome.parse::<usize>().unwrap_or_default();
        } else if let Some(false_outcome) = line.strip_prefix("    If false: throw to monkey ") {
            current_monkey.false_outcome = false_outcome.parse::<usize>().unwrap_or_default();
        }
    }

    monkeys.push(current_monkey);

    let lcm = monkeys.iter().fold(1, |acc, monkey| acc * monkey.test);

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].count += 1;

                let item = item % lcm;

                let worry_level = match monkeys[i].operation {
                    Operation::Addition(n) => item + n,
                    Operation::Multiplication(n) => item * n,
                    Operation::Square => item * item,
                };

                if worry_level % monkeys[i].test == 0 {
                    let jpp = monkeys[i].true_outcome;
                    monkeys[jpp].items.push_back(worry_level);
                } else {
                    let jpp = monkeys[i].false_outcome;
                    monkeys[jpp].items.push_back(worry_level);
                }
            }
        }
    }

    monkeys.sort_by(|a, b| a.count.partial_cmp(&b.count).unwrap());

    monkeys[monkeys.len() - 2].count * monkeys[monkeys.len() - 1].count
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
}
