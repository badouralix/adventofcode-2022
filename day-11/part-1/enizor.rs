use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

#[derive(Debug, Default, Clone, Copy)]
enum Op {
    Add(u64),
    Mul(u64),
    Double,
    #[default]
    Square,
}

#[derive(Debug, Default, Clone, Copy)]
struct Monkey {
    operation: Op,
    test: u64,
    target_true: usize,
    target_false: usize,
    inspected: u64,
}

#[derive(Debug, Default, Clone, Copy)]
struct Item {
    monkey: usize,
    worryness: u64,
}

#[derive(Debug, Default, Clone)]
struct MonkeyBand {
    items: Vec<Item>,
    band: Vec<Monkey>,
}

impl MonkeyBand {
    fn from_str(input: &str) -> Self {
        let mut items = Vec::new();
        let mut band = Vec::new();
        for s in input.trim().split("\n\n") {
            let mut lines = s.lines();
            let header = lines.next().unwrap();
            let monkey_num = header[7..header.len() - 1].parse().unwrap();
            for v in lines.next().unwrap()[18..].split(", ") {
                let item = Item {
                    monkey: monkey_num,
                    worryness: v.parse().unwrap(),
                };
                items.push(item);
            }
            let operation_line = lines.next().unwrap();
            let operand = match &operation_line[25..] {
                "old" => -1i64,
                s => s.parse().unwrap(),
            };
            let operation = match (operation_line.as_bytes()[23], operand) {
                (b'*', -1) => Op::Square,
                (b'*', v) => Op::Mul(v as u64),
                (b'+', -1) => Op::Double,
                (b'+', v) => Op::Add(v as u64),
                _ => panic!("Unsupported operation"),
            };
            let test = lines.next().unwrap()[21..].parse().unwrap();
            let target_true = lines.next().unwrap()[29..].parse().unwrap();
            let target_false = lines.next().unwrap()[30..].parse().unwrap();
            let monkey = Monkey {
                operation,
                test,
                target_true,
                target_false,
                inspected: 0,
            };
            band.resize(band.len().max(monkey_num + 1), Monkey::default());
            band[monkey_num] = monkey;
        }
        Self { items, band }
    }

    fn round(&mut self) {
        for i in 0..self.band.len() {
            let mut inspected = 0;
            let m = self.band[i];
            for item in &mut self.items {
                if item.monkey != i {
                    continue;
                }
                inspected += 1;
                match m.operation {
                    Op::Add(v) => item.worryness += v,
                    Op::Double => item.worryness *= 2,
                    Op::Mul(v) => item.worryness *= v,
                    Op::Square => item.worryness *= item.worryness,
                }
                item.worryness /= 3;
                item.monkey = if item.worryness % m.test == 0 {
                    m.target_true
                } else {
                    m.target_false
                };
            }
            self.band[i].inspected += inspected;
        }
    }

    fn monkey_business(&self) -> u64 {
        let mut max1 = 0;
        let mut max2 = 0;
        for m in &self.band {
            let mut i = m.inspected;
            if i > max1 {
                (i, max1) = (max1, i);
            }
            if i > max2 {
                max2 = i;
            }
        }
        max1 * max2
    }
}

fn run(input: &str) -> u64 {
    let mut monkeys = MonkeyBand::from_str(input);
    for _round in 0..20 {
        monkeys.round();
    }
    monkeys.monkey_business()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let example = "Monkey 0:
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
    If false: throw to monkey 1";
        assert_eq!(run(example), 10605)
    }
}
