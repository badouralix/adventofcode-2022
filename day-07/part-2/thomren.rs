fn main() {
    aoc::run(run)
}

const FS_TOTAL_SIZE: usize = 70000000;
const UPDATE_SIZE: usize = 30000000;

fn run(input: &str) -> usize {
    let dir_sizes = parse_dir_sizes(input);
    let available = FS_TOTAL_SIZE - dir_sizes.last().copied().unwrap_or_default();
    let min_deleted_size = UPDATE_SIZE - available;
    dir_sizes
        .into_iter()
        .filter(|&x| x > min_deleted_size)
        .min()
        .unwrap_or_default()
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
            24933642
        )
    }
}
