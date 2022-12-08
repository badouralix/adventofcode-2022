fn main() {
    aoc::run(run)
}

#[derive(Debug)]
struct Node {
    parent: Option<usize>,
    size: usize,
}

fn run(input: &str) -> usize {
    let root = Node {
        parent: None,
        size: 0,
    };

    let mut folders = Vec::from([root]);
    let mut current_folder = 0;

    for line in input.lines() {
        let words = line[..].split_whitespace().collect::<Vec<&str>>();
        if words[0] == "$" && words[1] == "cd" {
            if words[2] == "/" {
                current_folder = 0;
            } else if words[2] == ".." {
                current_folder = folders[current_folder].parent.unwrap_or_default();
            } else {
                let new_node = Node {
                    parent: Some(current_folder),
                    size: 0,
                };

                folders.push(new_node);
                current_folder = folders.len() - 1;
            }
        } else if words[0] != "dir" {
            let size = words[0].parse::<usize>().unwrap_or_default();
            folders[current_folder].size += size;

            let mut parent_folder = current_folder;
            while let Some(i) = folders[parent_folder].parent {
                folders[i].size += size;
                parent_folder = i;
            }
        }
    }

    let mut total_size = 0;
    for node in folders {
        if node.size <= 100000 {
            total_size += node.size;
        }
    }

    total_size
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
            95437
        )
    }
}
