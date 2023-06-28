use std::collections::HashSet;
use std::env::args;
use std::time::Instant;

struct Rock([Option<(usize, usize)>; 5]);

impl Rock {
    fn new_from_shape_id(id: usize, offset: usize) -> Rock {
        match id {
            0 =>
            /* #### */
            {
                Rock([
                    Some((2, offset)),
                    Some((3, offset)),
                    Some((4, offset)),
                    Some((5, offset)),
                    None,
                ])
            }
            1 =>
            /* .#.
             * ###
             * .#. */
            {
                Rock([
                    Some((3, offset + 2)),
                    Some((2, offset + 1)),
                    Some((3, offset + 1)),
                    Some((4, offset + 1)),
                    Some((3, offset)),
                ])
            }
            2 =>
            /* ..#
             * ..#
             * ### */
            {
                Rock([
                    Some((4, offset + 2)),
                    Some((4, offset + 1)),
                    Some((2, offset)),
                    Some((3, offset)),
                    Some((4, offset)),
                ])
            }
            3 =>
            /* #
             * #
             * #
             * # */
            {
                Rock([
                    Some((2, offset + 3)),
                    Some((2, offset + 2)),
                    Some((2, offset + 1)),
                    Some((2, offset)),
                    None,
                ])
            }
            4 =>
            /* ##
             * ## */
            {
                Rock([
                    Some((2, offset + 1)),
                    Some((3, offset + 1)),
                    Some((2, offset)),
                    Some((3, offset)),
                    None,
                ])
            }
            _ => unreachable!(),
        }
    }

    fn push_left_within_bound(&mut self, tower: &HashSet<(usize, usize)>) {
        for part in self.0.into_iter().flatten() {
            match part {
                (0, _) => return,
                (x, y) if tower.contains(&(x - 1, y)) => return,
                _ => {}
            }
        }

        for i in 0..self.0.len() {
            if let Some((x, y)) = self.0[i] {
                self.0[i] = Some((x - 1, y))
            }
        }
    }

    fn push_right_within_bound(&mut self, tower: &HashSet<(usize, usize)>) {
        for part in self.0.into_iter().flatten() {
            match part {
                (6, _) => return,
                (x, y) if tower.contains(&(x + 1, y)) => return,
                _ => {}
            }
        }

        for i in 0..self.0.len() {
            if let Some((x, y)) = self.0[i] {
                self.0[i] = Some((x + 1, y))
            }
        }
    }

    fn fall_or_stop(&mut self, tower: &HashSet<(usize, usize)>) -> bool {
        for part in self.0.into_iter().flatten() {
            match part {
                (_, 0) => return false,
                (x, y) if tower.contains(&(x, y - 1)) => return false,
                _ => {}
            }
        }

        for i in 0..self.0.len() {
            if let Some((x, y)) = self.0[i] {
                self.0[i] = Some((x, y - 1))
            }
        }

        true
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
    let jet_pattern = input.as_bytes();
    let mut jet_idx = 0;
    let mut tower = HashSet::with_capacity(50_000);
    let mut current_height = 0;

    for rock_idx in 0..2022 {
        let mut rock = Rock::new_from_shape_id(rock_idx % 5, current_height + 3);
        let mut falling = true;

        while falling {
            match jet_pattern[jet_idx] {
                b'<' => rock.push_left_within_bound(&tower),
                b'>' => rock.push_right_within_bound(&tower),
                _ => unreachable!(),
            }

            jet_idx += 1;
            if jet_idx >= jet_pattern.len() {
                jet_idx -= jet_pattern.len();
            }

            falling = rock.fall_or_stop(&tower)
        }

        for part in rock.0.into_iter().flatten() {
            tower.insert(part);

            if part.1 + 1 > current_height {
                current_height = part.1 + 1;
            }
        }
    }

    current_height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"), 3068)
    }
}
