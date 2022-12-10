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

    fn count_visible(&self) -> usize {
        let mut visibility = vec![false; self.width * self.length];
        let mut res = 2 * (self.width - 1 + self.length) - 4;
        // row visibility
        for j in 1..self.length - 1 {
            // skip first & last rows that are always visible up/down
            // left visibility
            visibility[j * self.width] = true;
            let mut max = self.at(0, j);
            for i in 1..self.width - 2 {
                // self.width-2 is visible from right
                let new = self.at(i, j);
                if new > max {
                    let visible = &mut visibility[j * self.width + i];
                    if !*visible {
                        res += 1;
                    }
                    *visible = true;
                    max = new;
                }
                if max == b'9' {
                    break;
                }
            }
            // right visibility
            visibility[j * self.width + self.width - 2] = true;
            let mut max = self.at(self.width - 2, j);
            for i in (1..self.width - 2).rev() {
                // 0 is visible from left
                let new = self.at(i, j);
                if new > max {
                    let visible = &mut visibility[j * self.width + i];
                    if !*visible {
                        res += 1;
                    }
                    *visible = true;
                    max = new;
                }
                if max == b'9' {
                    break;
                }
            }
        }
        // col visibility
        for i in 1..self.width - 2 {
            // skip first & last cols that are always visible left/right
            // up visibility
            visibility[i] = true;
            let mut max = self.at(i, 0);
            for j in 1..self.length - 1 {
                // self.length-1 is visible from down
                let new = self.at(i, j);
                if new > max {
                    let visible = &mut visibility[j * self.width + i];
                    if !*visible {
                        res += 1;
                    }
                    *visible = true;
                    max = new;
                }
                if max == b'9' {
                    break;
                }
            }
            // right visibility
            visibility[(self.length - 1) * (self.width)] = true;
            let mut max = self.at(i, self.length - 1);
            for j in (1..self.length - 1).rev() {
                // 0 is visible from up
                let new = self.at(i, j);
                if new > max {
                    let visible = &mut visibility[j * self.width + i];
                    if !*visible {
                        res += 1;
                    }
                    *visible = true;
                    max = new;
                }
                if max == b'9' {
                    break;
                }
            }
        }
        res
    }
}

fn run(input: &str) -> usize {
    let forest = Forest::from_str(input);
    forest.count_visible()
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
            21
        )
    }
}
