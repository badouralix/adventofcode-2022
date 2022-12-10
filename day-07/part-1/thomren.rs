use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

const MAX_DIR_SIZE: usize = 100000;

fn run(input: &str) -> usize {
    parse_dir_sizes(input).into_iter().filter(|&x| x < MAX_DIR_SIZE).sum()
}

fn parse_dir_sizes(input: &str) -> Vec<usize> {
    let mut stack: Vec<usize> = vec![];
    let mut dir_sizes: Vec<usize> = vec![];
    for line in input.lines() {
        if line.starts_with("$ cd") {
            let (_, path) = line.split_once(" cd ").expect("invalid cd command");
            match path {
                ".." => {
                    let s = stack.pop().unwrap_or_default();
                    dir_sizes.push(s);
                    if let Some(parent) = stack.last_mut() {
                        *parent += s;
                    }
                }
                _ => stack.push(0),
            }
        } else if !line.starts_with('$') && !line.starts_with("dir") {
            let (size, _) = line.split_once(' ').expect("invalid file");
            *stack.last_mut().unwrap() += size.parse::<usize>().expect("invalid file size");
        }
    }
    while let Some(s) = stack.pop() {
        dir_sizes.push(s);
        if let Some(parent) = stack.last_mut() {
            *parent += s;
        }
    }
    dir_sizes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
                .trim()),
            95437
        )
    }
}
