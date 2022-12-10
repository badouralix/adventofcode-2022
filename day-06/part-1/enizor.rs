use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

/// Counts the number of occurences of letters using 4 bits per letter.
#[derive(Clone, Copy, Default)]
struct Counter(u128);

// 1s everywhere except on bit pos = 0 mod 4
const MASK: u128 = {
    let mut v = 0xE;
    v |= v << 4;
    v |= v << 8;
    v |= v << 16;
    v |= v << 32;
    v |= v << 64;
    v
};

impl Counter {
    fn add(&mut self, b: u8) {
        let pos = (b - b'a') as usize * 4;
        self.0 += 1 << pos;
    }

    fn remove(&mut self, b: u8) {
        let pos = (b - b'a') as usize * 4;
        self.0 -= 1 << pos;
    }

    // any bit in the mask indicate a number >= 2
    fn all_different(&self) -> bool {
        self.0 & MASK == 0
    }
}

fn run(input: &str) -> usize {
    let bytes = input.as_bytes();
    const N: usize = 4;
    let mut counter = Counter::default();
    for &b in &bytes[0..N] {
        counter.add(b);
    }
    let mut p0 = 0;
    let mut p1 = N;
    loop {
        if counter.all_different() {
            return p1;
        } else {
            counter.remove(bytes[p0]);
            counter.add(bytes[p1]);
            p0 += 1;
            p1 += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7)
    }
}
