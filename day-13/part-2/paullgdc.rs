use std::cmp;
use std::env::args;
use std::time::Instant;

use aoc::paullgdc::tokenize::Tokenizer;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum ListElement {
    List(Vec<ListElement>),
    Number(i64),
}

impl cmp::PartialOrd for ListElement {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for ListElement {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        use cmp::Ordering::*;
        use ListElement::*;
        match (self, other) {
            (Number(left), Number(right)) => left.cmp(right),
            (Number(left), List(right)) => match &right[..] {
                [] => Greater,
                [right] => Number(*left).cmp(right),
                [right, _left_rest, ..] => match Number(*left).cmp(right) {
                    Greater => Greater,
                    Equal | Less => Less,
                },
            },
            (left @ List(..), right @ Number(..)) => right.cmp(left).reverse(),
            (List(left), List(right)) => left.cmp(right),
        }
    }
}

fn parse_list(tokenizer: &mut Tokenizer) -> Option<Vec<ListElement>> {
    let mut list = Vec::new();
    tokenizer.eat_byte(b'[')?;
    loop {
        list.push(match tokenizer.next_nth_byte(0)? {
            b']' => break,
            b'[' => ListElement::List(parse_list(tokenizer)?),
            _ => ListElement::Number(tokenizer.parse_next_decimal_i32()? as i64),
        });
        match tokenizer.next_nth_byte(0)? {
            b']' => break,
            b',' => tokenizer.advance(1),
            _ => return None,
        }
    }
    tokenizer.eat_byte(b']')?;
    Some(list)
}

fn parse_input(tokenizer: &mut Tokenizer) -> Option<Vec<(ListElement, ListElement)>> {
    let mut pairs = Vec::new();
    while !tokenizer.end() {
        let first = parse_list(tokenizer)?;
        tokenizer.eat_byte(b'\n')?;
        let second = parse_list(tokenizer)?;

        pairs.push((ListElement::List(first), ListElement::List(second)));
        tokenizer.eat_byte_or_end(b'\n')?;
        tokenizer.eat_byte_or_end(b'\n')?;
    }
    Some(pairs)
}

fn run(input: &str) -> usize {
    let mut tokenizer = Tokenizer::new(input.as_bytes());
    let lists = parse_input(&mut tokenizer).unwrap();
    let mut lists: Vec<_> = lists
        .into_iter()
        .flat_map(|(l, r)| [(l, false), (r, false)])
        .collect();
    lists.push((
        ListElement::List(vec![ListElement::List(vec![ListElement::Number(2)])]),
        true,
    ));
    lists.push((
        ListElement::List(vec![ListElement::List(vec![ListElement::Number(6)])]),
        true,
    ));
    lists.sort();
    lists
        .iter()
        .enumerate()
        .filter_map(|(i, (_, inserted))| if *inserted { Some(i + 1) } else { None })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"),
            140
        )
    }
}
