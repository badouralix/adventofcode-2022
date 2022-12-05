fn main() {
    aoc::run(run)
}

#[derive(Clone, Copy, Default, Debug)]
struct Crate {
    stack: u16,
    /// Position in its stack counting from the top
    pos: u16,
}

impl Crate {
    fn undo_crane(&mut self, moved: u16, from: u16, to: u16) {
        if self.stack == from {
            self.pos += moved;
        } else if self.stack == to {
            if self.pos < moved {
                self.stack = from;
            } else {
                self.pos -= moved;
            }
        }
    }
}

fn run(input: &str) -> String {
    let mut lines = input.lines();
    let mut stacks = Vec::with_capacity(16);
    for line in &mut lines {
        if line.len() == 0 {
            break;
        }
        let chars = line.as_bytes();
        let mut p = 0;
        while (p * 4 + 1) < line.len() {
            if (p + 1) >= stacks.len() {
                stacks.resize_with(p + 1, Vec::default);
            }
            let c = chars[p * 4 + 1];
            if c > b'A' {
                stacks[p].push(c as char);
            }
            p += 1;
        }
    }
    let nb_stacks = stacks.len();
    let mut crates = vec![Crate::default(); nb_stacks];
    for (i, c) in crates.iter_mut().enumerate() {
        c.stack = i as u16 + 1;
    }
    for line in &mut lines.rev() {
        let mut words = line.split_ascii_whitespace();
        let moved = words.nth(1).unwrap().parse::<u16>().unwrap();
        let from = words.nth(1).unwrap().parse::<u16>().unwrap();
        let to = words.nth(1).unwrap().parse::<u16>().unwrap();
        for c in crates.iter_mut() {
            c.undo_crane(moved, from, to);
        }
    }
    let mut res = String::with_capacity(nb_stacks);
    for c in crates.iter() {
        let v = stacks[c.stack as usize - 1][c.pos as usize];
        res.push(v);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"),
            "MCD"
        )
    }
}
