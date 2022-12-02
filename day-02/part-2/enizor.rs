fn main() {
    aoc::run(run)
}

fn run(input: &str) -> usize {
    let bytes = input.as_bytes();
    let mut total = 0;
    for i in 0..((bytes.len() + 1) / 4) {
        let s = bytes[i * 4 + 2] - b'X';
        let o = bytes[i * 4] - b'A';
        let win_points = 3 * s;
        let hand_points = ((o + s + 2) % 3) + 1;
        total += hand_points as usize + win_points as usize;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("A Y\nB X\nC Z"), 12)
    }
}
