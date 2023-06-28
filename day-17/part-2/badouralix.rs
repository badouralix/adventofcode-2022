use std::collections::HashSet;
use std::env::args;
use std::time::Instant;

struct Rock([Option<(isize, isize)>; 5]);

impl Rock {
    fn new_from_shape_id(id: usize, offset: isize) -> Rock {
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

    fn push_left_within_bound(&mut self, tower: &HashSet<(isize, isize)>) {
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

    fn push_right_within_bound(&mut self, tower: &HashSet<(isize, isize)>) {
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

    fn fall_or_stop(&mut self, tower: &HashSet<(isize, isize)>) -> bool {
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

fn run(input: &str) -> isize {
    // Store jet pattern as an array of char instead of a whole string
    let jet_pattern = input.as_bytes();
    // Keep an index of the current jet within the jet pattern
    let mut jet_idx = 0;
    // Initialize a set of all spaces containing a rock
    let mut tower = HashSet::with_capacity(50_000);
    // Initialize the maximum height reached by rocks
    let mut current_height = 0;
    // Initialize a vector of all indices in the jet pattern at which rocks stopped falling
    let mut jet_ids = Vec::new();

    for rock_idx in 0..1_000_000_000_000 {
        // Let the fall simulation run for the current rock
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

        // Save positions of all spaces containing rocks and update height accordingly
        let previous_height = current_height;
        for part in rock.0.into_iter().flatten() {
            tower.insert(part);

            if part.1 + 1 > current_height {
                current_height = part.1 + 1;
            }
        }

        // Compute the height gained with the fall of the current rock
        let diff_height = current_height - previous_height;
        // println!("{current_height} {diff_height}");

        // Store the index in jet pattern at which the rock stopped falling along with the gained height
        jet_ids.push((jet_idx, diff_height));

        // Try to find a cycle in the fall of all rocks
        for cycle_size in 1..jet_ids.len() / 2 {
            // A potential cycle has a size equals to an interval between two occurrences of the curren jet index
            if jet_ids[jet_ids.len() - 1] == jet_ids[jet_ids.len() - 1 - cycle_size] {
                // A potential cycle is an actual cycle only when the same sequence of jet indices occurs twice in a row
                let mut cycle = true;
                for i in 0..cycle_size {
                    if jet_ids[jet_ids.len() - 1 - i] != jet_ids[jet_ids.len() - 1 - cycle_size - i]
                    {
                        cycle = false;
                    }
                }

                // When a cycle is detected, the total height can be derived from the current height, the number of cycles until the last rock and the height of each cycle
                if cycle {
                    // println!("{cycle_size}\n{jet_ids:?}");

                    let number_of_remaining_rocks_to_fall = 1_000_000_000_000 - 1 - rock_idx;
                    let number_of_complete_cycles = number_of_remaining_rocks_to_fall / cycle_size;
                    let number_of_remaining_rocks_outside_cycles =
                        number_of_remaining_rocks_to_fall % cycle_size;
                    // println!("{rock_idx} {cycle_size} {number_of_remaining_rocks_to_fall} {number_of_complete_cycles} {number_of_remaining_rocks_outside_cycles}");

                    let mut total_height = current_height;
                    for i in 1..=cycle_size {
                        let diff_height = jet_ids[jet_ids.len() - 1 - cycle_size + i].1;
                        total_height += diff_height * number_of_complete_cycles as isize;
                        if i <= number_of_remaining_rocks_outside_cycles {
                            total_height += diff_height;
                        }
                    }

                    return total_height;
                }
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"),
            1514285714288
        )
    }
}
