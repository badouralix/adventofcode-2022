use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

const TOTAL_SPACE: u64 = 70000000;
const REQUIRED_SPACE: u64 = 30000000;

#[derive(Debug)]
enum DirContent {
    Dir(Box<DirKnowledge>),
    File(u64),
}

impl DirContent {
    fn size(&self) -> u64 {
        match self {
            Self::Dir(dircontent) => dircontent.size(),
            Self::File(size) => *size,
        }
    }

    fn answer(&self, cutoff: u64) -> (u64, u64) {
        match self {
            Self::Dir(dircontent) => dircontent.answer(cutoff),
            Self::File(size) => (u64::MAX, *size),
        }
    }

    fn get_dir_mut(&mut self) -> &mut DirKnowledge {
        match self {
            DirContent::Dir(knowledge) => knowledge.as_mut(),
            DirContent::File(..) => panic!("Wrong path!"),
        }
    }
}

#[derive(Debug)]
enum DirKnowledge {
    NoKnowledge,
    Known(Vec<(String, DirContent)>),
}

impl DirKnowledge {
    fn get_mut(&mut self, path: &[String]) -> &mut Self {
        if path.is_empty() {
            return self;
        }
        match self {
            DirKnowledge::Known(elems) => elems
                .iter_mut()
                .find_map(|x| {
                    if x.0 == path[0] {
                        Some(x.1.get_dir_mut().get_mut(&path[1..]))
                    } else {
                        None
                    }
                })
                .expect("Couldn't find folder"),
            DirKnowledge::NoKnowledge => panic!("Learning leaf {} before root", path[0]),
        }
    }

    fn size(&self) -> u64 {
        match self {
            Self::Known(vec) => vec.iter().fold(0, |acc, (_, t)| acc + t.size()),
            Self::NoKnowledge => panic!("Not everything is known"),
        }
    }

    fn answer(&self, cutoff: u64) -> (u64, u64) {
        match self {
            Self::Known(vec) => {
                let (sub_answer, size) = vec.iter().fold((u64::MAX, 0), |(x, y), (_, t)| {
                    let (a, b) = t.answer(cutoff);
                    if a >= cutoff && a < x {
                        (a, b + y)
                    } else {
                        (x, b + y)
                    }
                });
                if size < sub_answer && size >= cutoff {
                    (size, size)
                } else {
                    (sub_answer, size)
                }
            }
            Self::NoKnowledge => panic!("Not everything is known"),
        }
    }
}

struct ShellState {
    cwd: Vec<String>, // Current working directory, as a path from `/`
    root_knowledge: DirKnowledge,
}

impl ShellState {
    fn new() -> Self {
        Self {
            cwd: Vec::with_capacity(32),
            root_knowledge: DirKnowledge::NoKnowledge,
        }
    }

    // fn answer_p1(&self) -> u64 {
    //     self.root_knowledge.answer_p1()
    // }

    fn cd(&mut self, path: &str) {
        match path {
            "/" => {
                self.cwd.clear();
            }
            ".." => {
                self.cwd.pop();
            }
            _ => {
                self.cwd.push(path.to_string());
            }
        }
    }

    fn learn_ls<'a, I>(&mut self, knowledge: I) -> usize
    where
        I: Iterator<Item = &'a str>,
    {
        let mut count = 0;
        let new_knowledge = knowledge
            .map(|line| {
                count += 1;
                let (fst, snd) = line.split_once(' ').expect("Invalid input");
                if fst == "dir" {
                    (
                        snd.to_string(),
                        DirContent::Dir(Box::new(DirKnowledge::NoKnowledge)),
                    )
                } else {
                    let size = fst.parse().expect("Invalid input");
                    (snd.to_string(), DirContent::File(size))
                }
            })
            .collect();
        let current_knowledge = self.root_knowledge.get_mut(&self.cwd);
        *current_knowledge = DirKnowledge::Known(new_knowledge);
        count
    }
}

fn run(input: &str) -> u64 {
    let mut lines = input.split('\n');
    let mut shell = ShellState::new();
    while let Some(line) = lines.next() {
        if &line[2..4] == "cd" {
            shell.cd(&line[5..]);
        } else {
            let lines_clone = lines.clone().take_while(|x| !x.starts_with('$'));
            let count = shell.learn_ls(lines_clone);
            for _ in 0..count {
                lines.next();
            }
        }
    }
    let unused_space = TOTAL_SPACE - shell.root_knowledge.size();
    let required_space = REQUIRED_SPACE - unused_space;
    shell.root_knowledge.answer(required_space).0
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "$ cd /
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
7214296 k";
        assert_eq!(run(input), 24933642);
    }
}
