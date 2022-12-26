use std::{cell::RefCell, fmt::Display, rc::Rc};

struct File {
    name: String,
    size: u32,
}

pub struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    pub directories: Vec<Rc<RefCell<Directory>>>,
    files: Vec<File>,
}

#[derive(Debug)]
pub enum Command {
    List(Vec<ListItem>),
    ChangeDirectory(String),
}

impl Command {
    fn parse_prompt(input: &str) -> Command {
        if let Some(dir) = input.strip_prefix("$ cd ") {
            Command::ChangeDirectory(dir.to_string())
        } else if input.starts_with("$ ls") {
            Command::List(Vec::new())
        } else {
            unreachable!("invalid command {input}")
        }
    }
}

#[derive(Debug)]
pub enum ListItem {
    Directory(String),
    File(String, u32),
}

impl ListItem {
    fn parse(input: &str) -> Self {
        if input.starts_with("dir ") {
            return Self::Directory(
                input
                    .strip_prefix("dir ")
                    .expect("invalid dir syntax")
                    .to_string(),
            );
        }
        let file_parts = input.split_once(' ').expect("invalid file syntax");
        Self::File(
            file_parts.1.to_string(),
            file_parts.0.parse().expect("invalid file size"),
        )
    }
}

impl Directory {
    fn new(name: String, parent: Option<Rc<RefCell<Directory>>>) -> Self {
        Self {
            name,
            parent,
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

    fn root(start: Rc<RefCell<Directory>>) -> Rc<RefCell<Directory>> {
        let mut node = start;
        loop {
            let parent = node.borrow().parent.clone();
            match parent {
                Some(parent) => node = parent,
                None => return node,
            }
        }
    }

    pub fn from_commands(commands: Vec<Command>) -> Rc<RefCell<Self>> {
        let mut node = Rc::new(RefCell::new(Self::new("/".to_string(), None)));
        for command in commands {
            match command {
                Command::List(items) => {
                    for item in items {
                        match item {
                            ListItem::Directory(name) => {
                                let mut current = node.borrow_mut();
                                current.directories.push(
                                    Rc::new(RefCell::new(Directory {
                                        name,
                                        parent: Some(node.clone()),
                                        directories: Vec::new(),
                                        files: Vec::new(),
                                    }))
                                    .clone(),
                                );
                            }
                            ListItem::File(name, size) => {
                                let mut parent = node.borrow_mut();
                                parent.files.push(File { name, size })
                            }
                        }
                    }
                }
                Command::ChangeDirectory(dir) => match dir.as_str() {
                    ".." => {
                        let parent = node.borrow_mut().parent.clone();
                        node = parent.expect("no parent");
                        continue;
                    }
                    "/" => {
                        node = Directory::root(node);
                        continue;
                    }
                    dir => {
                        let dirs = node.borrow().directories.clone();
                        match dirs.iter().find(|child| child.borrow().name == dir) {
                            Some(child_dir) => node = child_dir.clone(),
                            None => unreachable!(
                                "could not find child {dir} under {}",
                                node.borrow().name
                            ),
                        }
                    }
                },
            }
        }
        Directory::root(node)
    }

    pub fn size(&self) -> u32 {
        return self.files.iter().fold(0, |acc, f| acc + f.size)
            + self
                .directories
                .iter()
                .fold(0, |acc, f| acc + f.borrow().size());
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for file in self.files.iter() {
            write!(f, "\n- {} (file, size={})", file.name, file.size)?;
        }
        for dir_ref in self.directories.iter() {
            let dir = dir_ref.borrow();
            let dir_output = dir.to_string();
            write!(f, "\n- {} (dir)", dir.name)?;
            for line in dir_output.lines().filter(|l| !l.is_empty()) {
                write!(f, "\n  {line}")?;
            }
        }
        Ok(())
    }
}

pub fn parse_commands(input: String) -> Vec<Command> {
    let mut lines = input.lines().peekable();
    // Ignore cd /
    lines.next().expect("no commands");
    let mut commands = Vec::new();
    loop {
        let prompt = lines.next().map(Command::parse_prompt);
        match prompt {
            Some(Command::List(_)) => {
                let mut items = Vec::<ListItem>::new();
                while let Some(item) = lines.next_if(|l| !l.starts_with('$')) {
                    items.push(ListItem::parse(item));
                }
                commands.push(Command::List(items));
            }
            Some(cd) => {
                commands.push(cd);
            }
            None => break,
        }
    }
    commands
}

pub fn run(input: String) -> u32 {
    let mut out = 0;
    let commands = parse_commands(input);
    let root = Directory::from_commands(commands);
    println!("{}", root.borrow());

    let mut stack = Vec::<Rc<RefCell<Directory>>>::new();
    stack.push(root);
    while let Some(node) = stack.pop() {
        let dir = node.borrow();
        let size = dir.size();
        if size < 100_000 {
            out += size;
        }
        for child in dir.directories.iter() {
            stack.push(child.clone());
        }
    }
    out
}

#[test]
fn example() {
    assert_eq!(
        run(r#"
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
7214296 k
    "#
        .trim()
        .to_string()),
        95437
    )
}
