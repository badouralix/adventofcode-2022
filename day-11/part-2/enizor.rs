use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(usize),
    Double,
    Mul(usize),
    Square,
}

impl Default for Op {
    fn default() -> Self {
        Op::Add(0)
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Monkey {
    operation: Op,
    test: usize,
    target_true: usize,
    target_false: usize,
    inspected: usize,
}

#[derive(Debug, Default, Clone, Copy)]
struct Item {
    monkey: usize,
    worryness: usize,
}

#[derive(Debug, Default, Clone)]
struct MonkeyBand {
    items: Vec<Item>,
    band: Vec<Monkey>,
    worry_lcm: usize,
}

impl MonkeyBand {
    fn from_str(input: &str) -> Self {
        let mut worry_lcm = 1;
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
                "old" => usize::MAX,
                s => s.parse().unwrap(),
            };
            let operation = match (operation_line.as_bytes()[23], operand) {
                (b'*', usize::MAX) => Op::Square,
                (b'*', v) => Op::Mul(v),
                (b'+', usize::MAX) => Op::Double,
                (b'+', v) => Op::Add(v),
                _ => panic!("Unsupported operation"),
            };
            let test = lines.next().unwrap()[21..].parse().unwrap();
            worry_lcm = lcm(worry_lcm, test);
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
        Self {
            items,
            band,
            worry_lcm,
        }
    }

    fn round(&mut self) {
        for i in 0..self.band.len() {
            let mut inspected = 0;
            for item in &mut self.items {
                if item.monkey != i {
                    continue;
                }
                inspected += 1;
                let m = self.band[i];
                match m.operation {
                    Op::Add(v) => item.worryness += v,
                    Op::Double => item.worryness *= 2,
                    Op::Mul(v) => item.worryness *= v,
                    Op::Square => item.worryness *= item.worryness,
                }
                item.worryness %= self.worry_lcm;
                item.monkey = if item.worryness % m.test == 0 {
                    m.target_true
                } else {
                    m.target_false
                };
            }
            self.band[i].inspected += inspected;
        }
    }

    fn monkey_business(&self) -> usize {
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

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if a > b {
        gcd(b, a)
    } else if a == b {
        a
    } else if a == 0 {
        b
    } else {
        gcd(a, b % a)
    }
}

fn run(input: &str) -> usize {
    let mut monkeys = MonkeyBand::from_str(input);
    for _round in 0..10000 {
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
        assert_eq!(run(example), 2713310158)
    }
}
