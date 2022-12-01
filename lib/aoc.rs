use std::{time::Instant, env::args, fmt};

pub mod paullgdc;

pub fn run<R: fmt::Display, F: FnOnce(&str) -> R>(f: F){
    let now = Instant::now();
    let output = f(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);    
}

