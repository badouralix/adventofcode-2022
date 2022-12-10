use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> usize {
    let mut load = 0;
    aoc::paullgdc::LastElem::new(input.lines())
        .flat_map(|(line, last)| {
            if line.is_empty() {
                let total = load;
                load = 0;
                return Some(total);
            }
            load += line.parse::<usize>().unwrap();
            last.then_some(load)
        })
        .max()
        .unwrap()
}
