use std::ops::Deref;

use aoc::{
    for_children,
    paullgdc::{
        arena::Handle,
        array::Array,
        tree::Tree,
        tokenize::{parse_decimal_u32, Tokenizer},
    },
};

fn main() {
    aoc::run(run)
}

type NodeName = Array<u8, 16>;

#[derive(Debug)]
enum Cmd {
    Cd { arg: NodeName },
    Ls { out: Vec<FsNode> },
}

#[derive(Debug)]
enum FsNode {
    File { name: NodeName, size: u32 },
    Dir { name: NodeName },
}

fn parse_output(input: &str) -> Vec<Cmd> {
    let mut commands = Vec::new();
    let mut tokenizer = Tokenizer::new(input.as_bytes());
    loop {
        if tokenizer.next_nth_byte(0).is_none() {
            break;
        }
        tokenizer.eat_byte(b'$').unwrap();
        let cmd_bin = tokenizer.consume_next_token().unwrap();
        commands.push(match cmd_bin {
            b"ls" => {
                tokenizer.eat_byte(b'\n');
                let mut output = Vec::new();
                while tokenizer.next_nth_byte(0) != Some(b'$')
                    && tokenizer.next_nth_byte(0).is_some()
                {
                    let next = tokenizer.consume_next_token().unwrap();
                    output.push(match next {
                        b"dir" => {
                            let name =
                                NodeName::from_slice(tokenizer.consume_next_token().unwrap())
                                    .unwrap();
                            FsNode::Dir { name }
                        }
                        size => {
                            let size = parse_decimal_u32(size).unwrap();
                            let name =
                                Array::from_slice(tokenizer.consume_next_token().unwrap()).unwrap();
                            FsNode::File { name, size }
                        }
                    });
                    tokenizer.eat_byte(b'\n');
                }
                Cmd::Ls { out: output }
            }
            b"cd" => {
                let arg = NodeName::from_slice(tokenizer.consume_next_token().unwrap()).unwrap();
                tokenizer.eat_byte(b'\n');
                Cmd::Cd { arg }
            }
            _ => {
                unreachable!()
            }
        })
    }
    commands
}

fn build_tree_from_output(commands: &[Cmd]) -> Option<Tree<FsNode>> {
    let mut tree = Tree::new();
    match &commands[0] {
        Cmd::Cd { arg } if arg.deref() == b"/" => {}
        _ => return None,
    }
    let root = tree.insert_node(FsNode::Dir {
        name: Array::from_slice(b"/").unwrap(),
    });
    let mut current_dir = root;
    for cmd in commands {
        match cmd {
            Cmd::Cd { arg } if arg.deref() == b"/" => {
                current_dir = root;
            }
            Cmd::Cd { arg } if arg.deref() == b".." => {
                current_dir = tree.get(current_dir).unwrap().parent;
            }
            Cmd::Cd { arg } => {
                let mut child = None;
                aoc::for_children!(c of node current_dir in graph tree {
                    match tree.get(c).unwrap().content {
                        FsNode::File { name, .. } if name.deref() == arg.deref() => panic!("cd on file"),
                        FsNode::File { .. } => {},
                        FsNode::Dir { name } if name.deref() == arg.deref() => { child = Some(c); break},
                        FsNode::Dir { .. } => {}
                    }
                });
                current_dir = match child {
                    Some(c) => c,
                    None => {
                        let c = tree.insert_node(FsNode::Dir { name: *arg });
                        tree.add_child(current_dir, c).unwrap();
                        c
                    }
                };
            }
            Cmd::Ls { out } => {
                'cmds_loop: for node in out {
                    match node {
                        FsNode::Dir { .. } => {}
                        FsNode::File { name, size } => {
                            aoc::for_children!(c of node current_dir in graph tree {
                                match tree.get(c).unwrap().content {
                                    FsNode::File { name: existing_name, ..} if name.deref() == existing_name.deref()  => {
                                        continue 'cmds_loop;
                                    },
                                    FsNode::File { .. } | FsNode::Dir { .. } => {},
                                }
                            });
                            let c = tree.insert_node(FsNode::File {
                                name: *name,
                                size: *size,
                            });
                            tree.add_child(current_dir, c).unwrap();
                        }
                    }
                }
            }
        }
    }
    tree.root = Some(root);
    Some(tree)
}

fn run(input: &str) -> u32 {
    let output = parse_output(input);
    let tree = build_tree_from_output(&output).unwrap();
    let root = tree.root.unwrap();


    fn visit_sum_size(tree: &Tree<FsNode>, node: Handle) -> u32 {
        let mut sum = 0;
        for_children!(c of node node in graph tree {
            match &tree.get(c).unwrap().content {
                FsNode::Dir { .. } => {
                    let dir_size = visit_sum_size(tree, c);
                    sum += dir_size;
                },
                FsNode::File { size, .. } => sum += size,
            }
        });


        sum
    }
    let total_size = visit_sum_size(&tree, root);

    const MAX_SIZE: u32 = 40000000;
    fn visit_min_directory(min_dir: &mut u32, total: u32, tree: &Tree<FsNode>, node: Handle) -> u32 {
        let mut sum = 0;
        for_children!(c of node node in graph tree {
            match &tree.get(c).unwrap().content {
                FsNode::Dir { .. } => {
                    let dir_size = visit_min_directory(min_dir, total, tree, c);
                    sum += dir_size;
                },
                FsNode::File { size, .. } => sum += size,
            }
        });
        if total - sum <= MAX_SIZE && sum < *min_dir {
            *min_dir = sum;
        }
        sum
    }
    let mut min_dir = u32::MAX;
    visit_min_directory(&mut min_dir, total_size, &tree, root);
    min_dir
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
