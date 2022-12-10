use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

struct Forest<'a> {
    bytes: &'a [u8],
    // beware width contains the aditional \n
    width: usize,
    length: usize,
}

impl<'a> Forest<'a> {
    fn from_str<'b: 'a>(s: &'b str) -> Self {
        let bytes = s.as_bytes();
        let width = s.find('\n').unwrap() + 1;
        let length = (1 + bytes.len()) / (width);
        Forest {
            bytes,
            width,
            length,
        }
    }

    fn at(&self, i: usize, j: usize) -> u8 {
        self.bytes[j * self.width + i]
    }

    fn score(&self, i: usize, j: usize) -> usize {
        let v = self.at(i, j);
        let mut left = 0;
        for k in (0..i).rev() {
            left += 1;
            if self.at(k, j) >= v {
                break;
            }
        }
        let mut right = 0;
        for k in i + 1..self.width - 1 {
            right += 1;
            if self.at(k, j) >= v {
                break;
            }
        }
        let mut up = 0;
        for k in (0..j).rev() {
            up += 1;
            if self.at(i, k) >= v {
                break;
            }
        }
        let mut down = 0;
        for k in j + 1..self.length {
            down += 1;
            if self.at(i, k) >= v {
                break;
            }
        }
        left * right * up * down
    }

    fn max_score(&self) -> usize {
        let mut res = 0;
        for j in 1..self.length - 1 {
            // skip first & last rows that have score 0
            for i in 1..self.width - 2 {
                let s = self.score(i, j);
                res = res.max(s)
            }
        }
        res
    }
}

fn run(input: &str) -> usize {
    let forest = Forest::from_str(input);
    forest.max_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("30373
25512
65332
33549
35390"),
            8
        )
    }
}
