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

#[derive(Debug)]
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

#[derive(Debug, Default)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Op,
    test: usize,
    target_true: usize,
    target_false: usize,
    inspected: usize,
}

impl Monkey {
    fn from_str(input: &str) -> Self {
        let mut lines = input.lines().skip(1);
        let items = lines.next().unwrap()[18..]
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect::<VecDeque<_>>();
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
        let target_true = lines.next().unwrap()[29..].parse().unwrap();
        let target_false = lines.next().unwrap()[30..].parse().unwrap();
        Self {
            items,
            operation,
            test,
            target_true,
            target_false,
            inspected: 0,
        }
    }
}

#[derive(Debug, Default)]
struct MonkeyBand {
    band: Vec<Monkey>,
    lcm: usize,
}

impl MonkeyBand {
    fn round(&mut self) {
        for i in 0..self.band.len() {
            let (left, right) = self.band.split_at_mut(i);
            let (m, right) = right.split_first_mut().unwrap();
            let mut inspected = 0;
            for mut item in m.items.drain(..) {
                inspected += 1;
                match m.operation {
                    Op::Add(v) => item += v,
                    Op::Double => item *= 2,
                    Op::Mul(v) => item *= v,
                    Op::Square => item *= item,
                }
                item %= self.lcm;
                let target_index = if item % m.test == 0 {
                    m.target_true
                } else {
                    m.target_false
                };
                let target_monkey = if target_index > left.len() {
                    &mut right[target_index - left.len() - 1]
                } else {
                    &mut left[target_index]
                };
                target_monkey.items.push_back(item);
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

    fn set_lcm(&mut self) {
        self.lcm = 1;
        for m in &self.band {
            self.lcm = lcm(self.lcm, m.test);
        }
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
    let mut monkeys = MonkeyBand::default();
    for m in input.trim().split("\n\n") {
        monkeys.band.push(Monkey::from_str(m));
    }
    monkeys.set_lcm();
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
