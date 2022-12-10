use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

const WINDOW_SIZE: usize = 14;

fn run(input: &str) -> usize {
    let input = input.as_bytes();
    let mut item_count = [0_u8; 27];
    let mut duplicate_count: u8 = 0;
    for c in input.iter().take(WINDOW_SIZE) {
        let c = &mut item_count[(*c - b'a') as usize];
        *c += 1;
        if *c == 2 {
            duplicate_count += 1;
        }
    }
    if duplicate_count == 0 {
        return WINDOW_SIZE;
    }
    input
        .windows(WINDOW_SIZE + 1)
        .enumerate()
        .filter_map(|(i, window)| {
            let previous = &mut item_count[(window[0] - b'a') as usize];
            *previous -= 1;
            if *previous == 1 {
                duplicate_count -= 1;
            }
            let next = &mut item_count[(window[WINDOW_SIZE] - b'a') as usize];
            *next += 1;
            if *next == 2 {
                duplicate_count += 1;
            }
            if duplicate_count == 0 {
                Some(i + 1 + WINDOW_SIZE)
            } else {
                None
            }
        })
        .next()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26)
    }
}
