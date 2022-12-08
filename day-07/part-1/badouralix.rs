use std::collections::HashMap;

#[derive(Debug)]
struct Filesystem {
    files: Option<HashMap<String, *mut Filesystem>>,
    parent: Option<*mut Filesystem>,
    size: Option<isize>,
}

impl Filesystem {
    fn new_from_input(input: &str) -> Filesystem {
        let mut root = Filesystem {
            files: Some(HashMap::new()),
            parent: None,
            size: None,
        };

        // The variable must be initialized as the rust compiler has no idea the first line of the input is `$ cd /`.
        let mut pwd: *mut Filesystem = &mut root;

        for line in input.split('\n') {
            println!("{}", line);
            unsafe {
                println!("{:?}", *pwd);
            }

            match line.chars().next() {
                Some('$') => {
                    if line == "$ cd /" {
                        // Same as the initial assignment of pwd.
                        pwd = &mut root;
                    } else if line == "$ cd .." {
                        // Required unsafe to dereference pwd raw pointer.
                        unsafe {
                            if let Some(parent) = (*pwd).parent {
                                pwd = parent
                            }
                        }
                    } else if line.starts_with("$ cd ") {
                        let dirname = String::from(line)[5..].to_string();
                        println!("{}", dirname);

                        // Required unsafe to dereference pwd raw pointer.
                        unsafe {
                            // We need to borrow with as_ref() as we cannot move out data behind a raw pointer.
                            pwd = (*pwd).files.as_ref().unwrap()[&dirname];
                        }
                    } else if line == "$ ls" {
                        continue;
                    }
                }

                Some(_) => {
                    if line.starts_with("dir") {
                        let dirname = String::from(line)[4..].to_string();
                        println!("found dir '{}'", dirname);

                        // Required unsafe to dereference pwd raw pointer.
                        unsafe {
                            // We need to borrow with as_mut() as we cannot move out data behind a raw pointer.
                            (*pwd).files.as_mut().unwrap().insert(
                                dirname,
                                &mut Filesystem {
                                    files: Some(HashMap::new()),
                                    parent: Some(pwd),
                                    size: None,
                                },
                            );
                        }

                        unsafe {
                            println!("{:?}", *pwd);
                        }
                    } else {
                        let mut split = line.split(' ');
                        let size: isize = split.next().unwrap().parse().unwrap();
                        let filename = split.next().unwrap().to_string();
                        println!("found file '{}'", filename);

                        // Required unsafe to dereference pwd raw pointer.
                        unsafe {
                            // We need to borrow with as_mut() as we cannot move out data behind a raw pointer.
                            (*pwd).files.as_mut().unwrap().insert(
                                filename,
                                &mut Filesystem {
                                    files: None,
                                    parent: None,
                                    size: Some(size),
                                },
                            );
                        }
                    }
                }

                None => unimplemented!(),
            }
        }

        root
    }

    fn total_size_of_at_most(&self, max_size: isize) -> isize {
        if let Some(size) = self.size {
            if size <= max_size {
                size
            } else {
                0
            }
        } else {
            let mut size = 0;
            for item in self.files.as_ref().unwrap().values() {
                unsafe {
                    size += (*(*item)).total_size_of_at_most(max_size);
                }

                if size > max_size {
                    return 0;
                }
            }

            if size <= max_size {
                size
            } else {
                0
            }
        }
    }
}

fn main() {
    aoc::run(run)
}

fn run(input: &str) -> isize {
    let root = Filesystem::new_from_input(input);
    root.total_size_of_at_most(100_000)
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
