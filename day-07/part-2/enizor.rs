use aoc::enizor::shell::Shell;

fn main() {
    aoc::run(run)
}

fn run(input: &str) -> usize {
    let mut shell = Shell::new();

    for command in input.split('$').skip(1) {
        shell.run(command)
    }
    let available = 70000000 - shell.root_size();
    let to_delete = 30000000 - available;
    shell
        .all_files()
        .iter()
        .filter_map(|f| {
            if !f.is_directory || f.size < to_delete {
                None
            } else {
                Some(f.size)
            }
        })
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("$ cd /
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
7214296 k"),
            24933642
        )
    }
}
