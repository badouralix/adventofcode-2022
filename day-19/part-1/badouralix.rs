use std::cmp::Ordering;
use std::collections::HashMap;
use std::env::args;
use std::ops::{Add, Sub};
use std::time::Instant;

#[derive(Debug)]
struct Blueprint {
    ore: Minerals,
    clay: Minerals,
    obsidian: Minerals,
    geode: Minerals,
}

impl Blueprint {
    fn new_from_line(line: &str) -> Blueprint {
        let split: Vec<&str> = line.split(' ').collect();
        Blueprint {
            ore: Minerals {
                ore: split[6].parse().unwrap(),
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            clay: Minerals {
                ore: split[12].parse().unwrap(),
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            obsidian: Minerals {
                ore: split[18].parse().unwrap(),
                clay: split[21].parse().unwrap(),
                obsidian: 0,
                geode: 0,
            },
            geode: Minerals {
                ore: split[27].parse().unwrap(),
                clay: 0,
                obsidian: split[30].parse().unwrap(),
                geode: 0,
            },
        }
    }

    fn run_optimal_for_n_minutes(
        &self,
        remaining_minutes: usize,
        minerals: Minerals,
        current_robots: Robots,
        ongoing_robots: Robots,
        cache: &mut HashMap<(usize, Minerals, Robots), Minerals>,
    ) -> Minerals {
        if remaining_minutes == 0 {
            return minerals;
        }

        // Finish building robots from the previous iteration
        let robots = current_robots + ongoing_robots;

        if let Some(v) = cache.get(&(remaining_minutes, minerals, robots)) {
            return *v;
        }

        // Try without building robots
        let mut result = self.run_optimal_for_n_minutes(
            remaining_minutes - 1,
            minerals + robots.into(),
            robots,
            Robots::default(),
            cache,
        );

        // Try to build an ore-collecting robot
        if self.ore <= minerals {
            let attempt = self.run_optimal_for_n_minutes(
                remaining_minutes - 1,
                minerals - self.ore + robots.into(),
                robots,
                Robots::new_with_ore(),
                cache,
            );
            if attempt.geode > result.geode {
                result = attempt;
            }
        }

        // Try to build a clay-collecting robot
        if self.clay <= minerals {
            let attempt = self.run_optimal_for_n_minutes(
                remaining_minutes - 1,
                minerals - self.clay + robots.into(),
                robots,
                Robots::new_with_clay(),
                cache,
            );
            if attempt.geode > result.geode {
                result = attempt;
            }
        }

        // Try to build an obsidian-collecting robot
        if self.obsidian <= minerals {
            let attempt = self.run_optimal_for_n_minutes(
                remaining_minutes - 1,
                minerals - self.obsidian + robots.into(),
                robots,
                Robots::new_with_obsidian(),
                cache,
            );
            if attempt.geode > result.geode {
                result = attempt;
            }
        }

        // Try to build a geode-collecting robot
        if self.geode <= minerals {
            let attempt = self.run_optimal_for_n_minutes(
                remaining_minutes - 1,
                minerals - self.geode + robots.into(),
                robots,
                Robots::new_with_geode(),
                cache,
            );
            if attempt.geode > result.geode {
                result = attempt;
            }
        }

        cache.insert((remaining_minutes, minerals, robots), result);
        result
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Minerals {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Add for Minerals {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

impl From<Robots> for Minerals {
    fn from(robot: Robots) -> Self {
        Minerals {
            ore: robot.ore,
            clay: robot.clay,
            obsidian: robot.obsidian,
            geode: robot.geode,
        }
    }
}

impl PartialOrd for Minerals {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.ore == other.ore
            && self.clay == other.clay
            && self.obsidian == other.obsidian
            && self.geode == other.geode
        {
            return Some(Ordering::Equal);
        }

        if self.ore <= other.ore
            && self.clay <= other.clay
            && self.obsidian <= other.obsidian
            && self.geode <= other.geode
        {
            return Some(Ordering::Less);
        }

        if self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
        {
            return Some(Ordering::Greater);
        }

        None
    }
}

impl Sub for Minerals {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Robots {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Robots {
    fn new_with_ore() -> Robots {
        Robots {
            ore: 1,
            ..Default::default()
        }
    }

    fn new_with_clay() -> Robots {
        Robots {
            clay: 1,
            ..Default::default()
        }
    }

    fn new_with_obsidian() -> Robots {
        Robots {
            obsidian: 1,
            ..Default::default()
        }
    }

    fn new_with_geode() -> Robots {
        Robots {
            geode: 1,
            ..Default::default()
        }
    }
}

impl Add for Robots {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
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
    let mut result = 0;
    let mut index = 1;

    for line in input.split('\n') {
        let blueprint = Blueprint::new_from_line(line);
        let cache = &mut HashMap::new();
        let minerals = blueprint.run_optimal_for_n_minutes(
            24,
            Minerals::default(),
            Robots::new_with_ore(),
            Robots::default(),
            cache,
        );

        result += index * minerals.geode;
        index += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
".trim()), 33)
    }
}
