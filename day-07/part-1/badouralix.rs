use std::{collections::HashMap, rc::Rc};

struct Filesystem {
    files: Option<HashMap<String, Rc<Filesystem>>>,
    parent: Option<Rc<Filesystem>>,
    size: Option<isize>,
}

fn main() {
    aoc::run(run)
}

fn run(input: &str) -> isize {
    // Wrap root into an rc so that it is allocated on the heap and can be referenced multiple times.
    // Options everywhere to distinguish files and dirs.
    let root = Rc::new(Filesystem {
        files: Some(HashMap::new()),
        parent: None,
        size: None,
    });

    // Here is the first reference to root. The rc is cloned without touching the underlying data.
    // The variable must be initialized as the rust compiler has no idea the first line of the input is `$ cd /`.
    let mut pwd = Rc::clone(&root);

    for line in input.split('\n') {
        match line.chars().next() {
            Some('$') => {
                if line == "$ cd /" {
                    // Same as the initial assignment of pwd.
                    pwd = Rc::clone(&root);
                } else if line == "$ cd .." {
                    // Apparently the borrowing with & is required here because Rc<Filesystem> does not implement Copy.
                    if let Some(parent) = &pwd.parent {
                        pwd = Rc::clone(parent)
                    }
                } else if line.starts_with("$ cd ") {
                    let dirname = String::from(line)[5..].to_string();

                    // We need to borrow the type's content with as_ref() as we cannot move out of an rc.
                    pwd = Rc::clone(&pwd.files.as_ref().unwrap()[&dirname]);
                } else if line == "$ ls" {
                    continue;
                }
            }

            Some(_) => {
                if line.starts_with("dir") {
                    let dirname = String::from(line)[5..].to_string();

                    // Here we are stuck and likely need a refcell...
                    // https://stackoverflow.com/questions/58599539/cannot-borrow-in-a-rc-as-mutable
                    pwd.files.as_mut().unwrap().insert(
                        dirname,
                        Rc::new(Filesystem {
                            files: Some(HashMap::new()),
                            parent: Some(Rc::clone(&pwd)),
                            size: None,
                        }),
                    );
                } else {
                    let mut split = line.split(' ');
                    let size: isize = split.next().unwrap().parse().unwrap();
                    let filename = split.next().unwrap().to_string();

                    // Here we are stuck and likely need a refcell...
                    // https://stackoverflow.com/questions/58599539/cannot-borrow-in-a-rc-as-mutable
                    pwd.files.as_mut().unwrap().insert(
                        filename,
                        Rc::new(Filesystem {
                            files: None,
                            parent: None,
                            size: Some(size),
                        }),
                    );
                }
            }

            None => unimplemented!(),
        }
    }
    0
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
