use std::collections::HashSet;

fn main() {
    aoc::run(run)
}

fn run(input: &str) -> usize {
    let mut visited = HashSet::new();

    let mut rope: [(isize, isize); 10] = [(0, 0); 10];

    let mut visited_count = 1;
    visited.insert((0, 0));

    for line in input.lines() {
        let c = line.split(' ').collect::<Vec<&str>>();
        let steps = c[1].parse::<isize>().unwrap_or_default();

        match c[0] {
            "U" => {
                for _ in 0..steps {
                    rope[0].1 += 1;

                    for i in 1..10 {
                        if rope[i - 1].1 - rope[i].1 > 1 {
                            rope[i].1 += 1;

                            if rope[i - 1].0.abs_diff(rope[i].0) >= 1 {
                                rope[i].0 = rope[i - 1].0;
                            }

                            if i == 9 && !visited.contains(&rope[9]) {
                                visited.insert(rope[9]);
                                visited_count += 1;
                            }
                        }
                    }
                }
            }
            "D" => {
                for _ in 0..steps {
                    rope[0].1 -= 1;

                    for i in 1..10 {
                        if rope[i].1 - rope[i - 1].1 > 1 {
                            rope[i].1 -= 1;

                            if rope[i - 1].0.abs_diff(rope[i].0) >= 1 {
                                rope[i].0 = rope[i - 1].0;
                            }

                            if i == 9 && !visited.contains(&rope[9]) {
                                visited.insert(rope[9]);
                                visited_count += 1;
                            }
                        }
                    }
                }
            }
            "R" => {
                for _ in 0..steps {
                    rope[0].0 += 1;

                    for i in 1..10 {
                        if rope[i - 1].0 - rope[i].0 > 1 {
                            rope[i].0 += 1;

                            if rope[i - 1].1.abs_diff(rope[i].1) >= 1 {
                                rope[i].1 = rope[i - 1].1;
                            }

                            if i == 9 && !visited.contains(&rope[9]) {
                                visited.insert(rope[9]);
                                visited_count += 1;
                            }
                        }
                    }
                }
            }
            "L" => {
                for _ in 0..steps {
                    rope[0].0 -= 1;

                    for i in 1..10 {
                        if rope[i].0 - rope[i - 1].0 > 1 {
                            rope[i].0 -= 1;

                            if rope[i - 1].1.abs_diff(rope[i].1) >= 1 {
                                rope[i].1 = rope[i - 1].1;
                            }

                            if i == 9 && !visited.contains(&rope[9]) {
                                visited.insert(rope[9]);
                                visited_count += 1;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    visited_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        // assert_eq!(
        //     run("R 4
        // U 4
        // L 3
        // D 1
        // R 4
        // D 1
        // L 5
        // R 2"),
        //     1
        // );

        assert_eq!(
            run("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"),
            36
        )
    }
}
