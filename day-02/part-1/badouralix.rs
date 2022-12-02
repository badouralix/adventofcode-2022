#[derive(PartialEq)]
enum HandShape {
    Paper,
    Rock,
    Scissors,
}

impl TryFrom<isize> for HandShape {
    type Error = ();

    fn try_from(v: isize) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(HandShape::Rock),
            2 => Ok(HandShape::Paper),
            3 => Ok(HandShape::Scissors),
            _ => Err(()),
        }
    }
}

fn main() {
    aoc::run(run)
}

fn run(input: &str) -> isize {
    let mut result = 0;

    for line in input.split('\n') {
        let opponent = line.chars().next().unwrap() as isize - 'A' as isize + 1;
        let player = line.chars().nth(2).unwrap() as isize - 'X' as isize + 1;

        result += player;

        if is_draw(opponent.try_into().unwrap(), player.try_into().unwrap()) {
            result += 3
        } else if is_win(opponent.try_into().unwrap(), player.try_into().unwrap()) {
            result += 6
        }
    }

    result
}

fn is_draw(opponent: HandShape, player: HandShape) -> bool {
    opponent == player
}

fn is_win(opponent: HandShape, player: HandShape) -> bool {
    match opponent {
        HandShape::Rock => player == HandShape::Paper,
        HandShape::Paper => player == HandShape::Scissors,
        HandShape::Scissors => player == HandShape::Rock,
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("
A Y
B X
C Z
        "
            .trim()),
            15
        )
    }
}
