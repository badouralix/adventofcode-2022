fn main() {
    aoc::run(run)
}

fn run(input: &str) -> isize {
    let mut score = 0;
    for line in input.split('\n') {
        let (left, right) = line.split_once(' ').unwrap();
        let (left, right) = (
            Janken::parse_left(left).unwrap(),
            MatchRes::parse(right).unwrap(),
        );
        let match_score = right.other_player_score(&left);
        score += match_score
            + match right {
                MatchRes::Win => 6,
                MatchRes::Draw => 3,
                MatchRes::Loss => 0,
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

    fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

enum MatchRes {
    Win,
    Draw,
    Loss,
}

impl MatchRes {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => return None,
        })
    }

    fn result(&self) -> i32 {
        match self {
            Self::Draw => 0,
            Self::Win => 1,
            Self::Loss => 2,
        }
    }

    fn other_player_score(&self, player: &Janken) -> i32 {
        (self.result() + (player.score() - 1)).rem_euclid(3) + 1
    }
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
            12
        )
    }
}
