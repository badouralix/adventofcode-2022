fn main() {
    aoc::run(run)
}

fn run(input: &str) -> isize {
    let mut score = 0;
    for line in input.split('\n') {
        let (left, right) = line.split_once(' ').unwrap();
        let (left, right) = (
            Janken::parse_left(left).unwrap(),
            Janken::parse_right(right).unwrap(),
        );
        match right.match_result(&left) {
            MatchRes::Win => score += right.score() + 6,
            MatchRes::Draw => score += right.score() + 3,
            MatchRes::Loss => score += right.score(),
        }
    }
    score as isize
}

enum Janken {
    Rock,
    Paper,
    Scissors,
}

impl Janken {
    fn parse_left(s: &str) -> Option<Self> {
        Some(match s {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => return None,
        })
    }

    fn parse_right(s: &str) -> Option<Self> {
        Some(match s {
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => return None,
        })
    }

    fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn match_result(&self, other: &Self) -> MatchRes {
        match ((self.score() - 1) - (other.score() - 1)).rem_euclid(3) {
            0 => MatchRes::Draw,
            1 => MatchRes::Win,
            2 => MatchRes::Loss,
            _ => unimplemented!(),
        }
    }
}

enum MatchRes {
    Win,
    Draw,
    Loss,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("A Y
B X
C Z"),
            15
        )
    }
}
