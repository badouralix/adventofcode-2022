use std::collections::HashSet;

fn main() {
    aoc::run(run)
}

fn run(input: &str) -> usize {
    let mut visited = HashSet::new();

    let (mut head_x, mut head_y): (isize, isize) = (0, 0);
    let (mut tail_x, mut tail_y): (isize, isize) = (0, 0);
    let mut visited_count = 1;
    visited.insert((0, 0));

    for line in input.lines() {
        let c = line.split(' ').collect::<Vec<&str>>();
        let steps = c[1].parse::<isize>().unwrap_or_default();

        match c[0] {
            "U" => {
                for _ in 0..steps {
                    head_y += 1;

                    if head_y - tail_y > 1 {
                        tail_y += 1;
                        tail_x = head_x;

                        if !visited.contains(&(tail_y, tail_x)) {
                            visited.insert((tail_y, tail_x));
                            visited_count += 1;
                        }
                    }
                }
            }
            "D" => {
                for _ in 0..steps {
                    head_y -= 1;

                    if tail_y - head_y > 1 {
                        tail_y -= 1;
                        tail_x = head_x;

                        if !visited.contains(&(tail_y, tail_x)) {
                            visited.insert((tail_y, tail_x));
                            visited_count += 1;
                        }
                    }
                }
            }
            "R" => {
                for _ in 0..steps {
                    head_x += 1;

                    if head_x - tail_x > 1 {
                        tail_x += 1;
                        tail_y = head_y;

                        if !visited.contains(&(tail_y, tail_x)) {
                            visited.insert((tail_y, tail_x));
                            visited_count += 1;
                        }
                    }
                }
            }
            "L" => {
                for _ in 0..steps {
                    head_x -= 1;

                    if tail_x - head_x > 1 {
                        tail_x -= 1;
                        tail_y = head_y;

                        if !visited.contains(&(tail_y, tail_x)) {
                            visited.insert((tail_y, tail_x));
                            visited_count += 1;
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
        assert_eq!(
            run("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"),
            13
        )
    }
}
