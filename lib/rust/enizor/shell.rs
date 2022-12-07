use std::fmt;

type FileHandle = usize;

#[derive(Default)]
pub struct FileSystem<'a> {
    files: Vec<File<'a>>,
    root: FileHandle,
}

#[derive(Debug, Default)]
pub struct File<'a> {
    pub name: &'a str,
    pub size: usize,
    pub children: Vec<FileHandle>,
    pub parent: Option<FileHandle>,
    pub is_directory: bool,
}

impl<'a> File<'a> {
    // fn
}

impl<'a> FileSystem<'a> {
    fn get_node_mut(&mut self, node: FileHandle) -> &mut File<'a> {
        &mut self.files[node]
    }

    fn get_node(&self, node: FileHandle) -> &File<'a> {
        &self.files[node]
    }

    fn new_node<'b: 'a>(
        &mut self,
        name: &'b str,
        parent: FileHandle,
        size: Option<usize>,
    ) -> FileHandle {
        let handle = self.files.len();
        self.files.push(File {
            name,
            size: size.unwrap_or(0),
            children: Vec::new(),
            parent: Some(parent),
            is_directory: size.is_none(),
        });
        handle
    }

    fn new() -> Self {
        FileSystem {
            files: vec![File {
                name: "/",
                size: 0,
                children: Vec::new(),
                parent: None,
                is_directory: true,
            }],
            root: 0,
        }
    }

    fn propagate_size(&mut self, handle: FileHandle, size: usize) {
        let node = self.get_node_mut(handle);
        node.size += size;
        if let Some(parent) = node.parent {
            self.propagate_size(parent, size);
        }
    }

    fn pretty_print(
        &self,
        node: FileHandle,
        offset: usize,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let file = self.get_node(node);
        write!(f, "{:<9} ", file.size)?;
        for _ in 0..offset {
            write!(f, "  ")?;
        }
        write!(f, "{}", file.name)?;
        if file.is_directory {
            writeln!(f, "/")?;
            for &c in &file.children {
                self.pretty_print(c, offset + 1, f)?;
            }
        } else {
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<'a> fmt::Debug for FileSystem<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(0, 0, f)
    }
}

pub struct Shell<'a> {
    fs: FileSystem<'a>,
    current_dir: FileHandle,
}

impl<'a> Shell<'a> {
    pub fn new() -> Self {
        Self {
            fs: FileSystem::new(),
            current_dir: 0,
        }
    }

    pub fn run<'b: 'a>(&mut self, command: &'b str) {
        match &command[1..3] {
            "cd" => self.cd(&command[4..]),
            "ls" => self.ls(&command[4..]),
            x => panic!("Unknown command {}", x),
        }
    }

    fn cd<'b: 'a>(&mut self, name: &'b str) {
        match name.trim() {
            "/" => self.current_dir = self.fs.root,
            ".." => {
                self.current_dir = self
                    .fs
                    .get_node(self.current_dir)
                    .parent
                    .expect("Current dir has no parent!");
            }
            name => {
                for &node in self.fs.get_node(self.current_dir).children.iter() {
                    let n = self.fs.get_node(node);
                    if n.is_directory && n.name == name {
                        self.current_dir = node;
                        return;
                    }
                }
                panic!(
                    "Current directory {} has no subdirectory {}",
                    self.fs.get_node(self.current_dir).name,
                    name
                );
            }
        }
    }

    fn ls<'b: 'a>(&mut self, out: &'b str) {
        let mut size = None;
        for (i, word) in out.split_ascii_whitespace().enumerate() {
            if i % 2 == 0 {
                match word {
                    "dir" => size = None,
                    s => size = Some(s.parse::<usize>().expect("Failed to parse file size")),
                }
            } else {
                let parent = self.current_dir;
                let handle = self.fs.new_node(word, parent, size);
                self.fs.get_node_mut(parent).children.push(handle);
                if let Some(s) = size {
                    self.fs.propagate_size(parent, s);
                }
            }
        }
    }

    pub fn all_files(&self) -> &[File<'a>] {
        &self.fs.files
    }

    pub fn root_size(&self) -> usize {
        self.fs.get_node(0).size
    }

    pub fn fs(&self) -> &FileSystem<'a> {
        &self.fs
    }
}

impl<'a> Default for Shell<'a> {
    fn default() -> Self {
        Self::new()
    }
}
