use core::fmt;
use std::env::args;
use std::fmt::Display;
use std::time::Instant;

struct Monkey {
    id: usize,
    items: Vec<isize>,
    test_divisible_by: isize,
    test_dst_monkey_false: usize,
    test_dst_monkey_true: usize,
    test_op_add: Option<isize>,
    test_op_mul: Option<isize>,
    test_op_square: bool,
}

impl Monkey {
    fn new(definition: &str) -> Monkey {
        let mut lines = definition.split('\n');

        let id = lines.next().unwrap().as_bytes()[8] as usize - b'0' as usize;

        let items = lines
            .next()
            .unwrap()
            .chars()
            .skip(18)
            .collect::<String>()
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();

        let operation = lines.next().unwrap();
        let mut test_op_add = None;
        let mut test_op_mul = None;
        let test_op_square = operation.contains("old * old");
        if !test_op_square {
            if operation.contains('+') {
                test_op_add = Some(
                    operation
                        .chars()
                        .skip(25)
                        .collect::<String>()
                        .parse()
                        .unwrap(),
                )
            } else if operation.contains('*') {
                test_op_mul = Some(
                    operation
                        .chars()
                        .skip(25)
                        .collect::<String>()
                        .parse()
                        .unwrap(),
                )
            } else {
                unimplemented!()
            }
        }

        let test_divisible_by = lines
            .next()
            .unwrap()
            .chars()
            .skip(21)
            .collect::<String>()
            .parse()
            .unwrap();

        let test_dst_monkey_true = lines.next().unwrap().as_bytes()[29] as usize - b'0' as usize;

        let test_dst_monkey_false = lines.next().unwrap().as_bytes()[30] as usize - b'0' as usize;

        Monkey {
            id,
            items,
            test_divisible_by,
            test_dst_monkey_false,
            test_dst_monkey_true,
            test_op_add,
            test_op_mul,
            test_op_square,
        }
    }

    fn throw(&self, old: isize) -> (isize, usize) {
        let new = if let Some(v) = self.test_op_add {
            (old + v) / 3
        } else if let Some(v) = self.test_op_mul {
            (old * v) / 3
        } else if self.test_op_square {
            (old * old) / 3
        } else {
            unimplemented!()
        };

        if new % self.test_divisible_by == 0 {
            (new, self.test_dst_monkey_true)
        } else {
            (new, self.test_dst_monkey_false)
        }
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Monkey {}: {:?}", self.id, self.items)
    }
}

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> usize {
    let mut inspects = Vec::new();
    let mut monkeys = Vec::new();
    for (idx, definition) in input.split("\n\n").enumerate() {
        assert!(definition.starts_with(&format!("Monkey {idx}:\n")));
        inspects.push(0);
        monkeys.push(Monkey::new(definition));
    }

    for _ in 0..20 {
        for src in 0..monkeys.len() {
            let items = monkeys[src].items.to_owned();
            for item in items {
                let (new, dst) = monkeys[src].throw(item);
                monkeys[dst].items.push(new);
            }
            inspects[src] += monkeys[src].items.len();
            monkeys[src].items.clear();
        }

        // for monkey in &monkeys {
        //     println!("{monkey}");
        // }
        // println!()
    }

    // for (i, inspect) in inspects.iter().enumerate() {
    //     println!("Monkey {} inspected items {} times.", i, inspect);
    // }
    // println!();

    inspects.sort();
    inspects.reverse();
    inspects[0] * inspects[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("
Monkey 0:
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
    If false: throw to monkey 1"
                .trim()),
            10605
        )
    }
}
