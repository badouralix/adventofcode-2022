fn main() {
    aoc::run(run)
}

fn run(input: &str) -> isize {
    const N: usize = 4;

    let mut total = 0;
    let mut chunks = input.as_bytes().chunks_exact(N);
    for round in &mut chunks {
        let round: [u8; N] = unsafe { round.try_into().unwrap_unchecked() };
        total += score(round) as isize;
    }
    let mut last_round = [0; 4];
    last_round[..chunks.remainder().len()].copy_from_slice(chunks.remainder());
    total += score(last_round) as isize;

    total
}


fn score(round: [u8; 4]) -> u8 {
    (round[0] - b'A' + (round[2] - b'X') + 2) % 3 + 1 + (round[2] - b'X') * 3
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
