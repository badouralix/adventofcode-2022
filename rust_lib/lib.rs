use std::{fmt::Display, time::Instant, env::args};

pub mod enizor;
pub mod paullgdc;

pub fn run<R: Display, F: FnOnce(&str) -> R>(f: F){
    let now = Instant::now();
    let output = f(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);    
}
