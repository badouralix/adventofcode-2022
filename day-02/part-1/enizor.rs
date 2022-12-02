fn main() {
    aoc::run(run)
}

const A: u64 = b'A' as u64;
const X: u64 = b'X' as u64 - 1;
const SELF_DIFF: u64 =
    (X << 56) + (X << 48) + (X << 40) + (X << 32) + (X << 24) + (X << 16) + (X << 8) + X;
const OTHER_DIFF: u64 =
    (A << 56) + (A << 48) + (A << 40) + (A << 32) + (A << 24) + (A << 16) + (A << 8) + A;

// sum all 16 lanes of 4 bits
fn simd_fold(mut p: u64) -> u64 {
    // sum into 8 lanes of 8 bits
    p = ((p >> 4) & 0x0F0F0F0F0F0F0F0F) + (p & 0x0F0F0F0F0F0F0F0F);
    // sum into 4 lanes of 16 bits
    p = ((p >> 8) & 0x00FF00FF00FF00FF) + (p & 0x00FF00FF00FF00FF);
    // sum into 2 lanes of 32 bits
    p = ((p >> 16) & 0x0000FFFF0000FFFF) + (p & 0x0000FFFF0000FFFF);
    // sum into 1 lanes of 64 bits
    p = ((p >> 32) & 0x00000000FFFFFFFF) + (p & 0x00000000FFFFFFFF);
    p
}

fn run(input: &str) -> u64 {
    let bytes = input.as_bytes();
    let mut total = 0;
    let mut i = 0;
    while i + 60 < bytes.len() {
        let mut self_packed_1: u64 = ((bytes[i + 2] as u64) << 56)
            + ((bytes[i + 6] as u64) << 48)
            + ((bytes[i + 10] as u64) << 40)
            + ((bytes[i + 14] as u64) << 32)
            + ((bytes[i + 18] as u64) << 24)
            + ((bytes[i + 22] as u64) << 16)
            + ((bytes[i + 26] as u64) << 8)
            + (bytes[i + 30] as u64);
        let mut other_packed_1: u64 = ((bytes[i] as u64) << 56)
            + ((bytes[i + 4] as u64) << 48)
            + ((bytes[i + 8] as u64) << 40)
            + ((bytes[i + 12] as u64) << 32)
            + ((bytes[i + 16] as u64) << 24)
            + ((bytes[i + 20] as u64) << 16)
            + ((bytes[i + 24] as u64) << 8)
            + (bytes[i + 28] as u64);
        self_packed_1 -= SELF_DIFF;
        other_packed_1 -= OTHER_DIFF;
        let mut self_packed_2: u64 = ((bytes[i + 34] as u64) << 56)
            + ((bytes[i + 38] as u64) << 48)
            + ((bytes[i + 42] as u64) << 40)
            + ((bytes[i + 46] as u64) << 32)
            + ((bytes[i + 50] as u64) << 24)
            + ((bytes[i + 54] as u64) << 16)
            + ((bytes[i + 58] as u64) << 8)
            + (bytes[i + 62] as u64);
        let mut other_packed_2: u64 = ((bytes[i + 32] as u64) << 56)
            + ((bytes[i + 36] as u64) << 48)
            + ((bytes[i + 40] as u64) << 40)
            + ((bytes[i + 44] as u64) << 32)
            + ((bytes[i + 48] as u64) << 24)
            + ((bytes[i + 52] as u64) << 16)
            + ((bytes[i + 56] as u64) << 8)
            + (bytes[i + 60] as u64);
        self_packed_2 -= SELF_DIFF;
        other_packed_2 -= OTHER_DIFF;
        // s + 1
        let self_packed = (self_packed_1 << 4) + self_packed_2;
        // o
        let other_packed = (other_packed_1 << 4) + other_packed_2;
        let hand_points = self_packed;
        const THREES: u64 = 0x3333333333333333;
        // win_points = 3*((3 + (s+1) - o)%3)
        // 3 + s + 1 - o
        let mut packed = THREES + self_packed - other_packed;
        // compute mod3 by adding digits in each 4-bit lane
        // each lane of 4 bits is in [2,6] => digits sum is <= 3
        packed = ((packed >> 2) & THREES) + (packed & THREES);
        // transform each 3 lane into a 0
        packed = ((((packed + 0x1111111111111111) >> 2) & THREES) + packed) & THREES;
        // now multiply by 3 to get win points
        packed += packed << 1;
        // add back hand points
        packed += hand_points;
        // fold and save
        total += simd_fold(packed);
        i += 64;
    }
    // simple loop for remainder
    while i + 2 < bytes.len() {
        let s_1 = bytes[i + 2] - b'X' + 1;
        // s+1
        let hand_points = s_1;
        // win_points = 3*((3 + (s+1) - o)%3)
        let o = bytes[i] - b'A';
        let win_points = 3 * ((3 + s_1 - o) % 3);
        total += hand_points as u64 + win_points as u64;
        i += 4;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("A Y\nB X\nC Z\nB Y\n"), 20);
        assert_eq!(simd_fold(0xFFFFFFFFFFFFFFFF), 16 * 15);
        assert_eq!(simd_fold(0x3366060636666606), 27 + 7 * 6);
        // use at least 16 games to test SIMD
        assert_eq!(
            run("A X
A Y
A Z
B X
B Y
B Z
C X
C Y
C Z
A Y
A Y
A Y
A Y
A Y
A Y
A Y
A Y
A Y"),
            27 + 54 + 18 + 18
        )
    }
}
