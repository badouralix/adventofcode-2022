use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

#[derive(Clone, Debug, PartialEq)]
enum Packet {
    Integer(usize),
    List(Vec<Packet>),
}

impl Packet {
    fn new_from_line(line: &str) -> Packet {
        let mut tokens = Vec::new();
        let mut acc = None;

        for c in line.chars() {
            match c {
                '[' | ']' | ',' => {
                    if let Some(x) = acc {
                        tokens.push(x);
                        acc = None;
                    }
                    if c != ',' {
                        tokens.push(c.to_string())
                    }
                }
                _ => {
                    if let Some(x) = acc {
                        acc = Some(format!("{x}{c}"))
                    } else {
                        acc = Some(c.to_string())
                    }
                }
            }
        }

        Packet::new_from_tokens(&tokens, 1).0
        // println!("{:?}", line);
        // println!("{:?}", tokens);
        // let (packet, _) = Packet::new_from_tokens(&tokens, 1);
        // println!("{:?}", packet);
        // println!();
        // packet
    }

    fn new_from_tokens(tokens: &Vec<String>, init: usize) -> (Packet, usize) {
        let mut idx = init;
        let mut list = Vec::new();

        while idx < tokens.len() {
            if tokens[idx] == "[" {
                let packet;
                (packet, idx) = Packet::new_from_tokens(tokens, idx + 1);
                list.push(packet);
            } else if tokens[idx] == "]" {
                return (Packet::List(list), idx + 1);
            } else {
                list.push(Packet::Integer(tokens[idx].parse().unwrap()));
                idx += 1
            }
        }

        (Packet::List(list), idx)
    }

    fn less_or_dunno(right: Packet, left: Packet) -> Option<bool> {
        match (right, left) {
            (Packet::Integer(r), Packet::Integer(l)) if r == l => None,
            (Packet::Integer(r), Packet::Integer(l)) => Some(r < l),
            (Packet::Integer(r), Packet::List(l)) => {
                Packet::less_or_dunno(Packet::List(vec![Packet::Integer(r)]), Packet::List(l))
            }
            (Packet::List(r), Packet::Integer(l)) => {
                Packet::less_or_dunno(Packet::List(r), Packet::List(vec![Packet::Integer(l)]))
            }
            (Packet::List(r), Packet::List(l)) => {
                for i in 0..r.len() {
                    if i >= l.len() {
                        return Some(false);
                    } else if let Some(b) = Packet::less_or_dunno(r[i].clone(), l[i].clone()) {
                        return Some(b);
                    }
                }

                if r.len() == l.len() {
                    return None;
                }

                Some(true)
            }
        }
    }
}

fn run(input: &str) -> usize {
    let mut result = 0;

    for (idx, pair) in input.split("\n\n").enumerate() {
        let (right, left) = pair.split_once('\n').unwrap();
        if Packet::less_or_dunno(Packet::new_from_line(right), Packet::new_from_line(left)).unwrap()
        {
            result += idx + 1;
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
[1,1,3,1,1]
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
[1,[2,[3,[4,[5,6,0]]]],8,9]"
                .trim()),
            13
        )
    }
}
