use std::{cell::RefCell, rc::Rc};

use crate::part1::{parse_commands, Directory};

const TOTAL_DISK_SPACE: u32 = 70_000_000;
const REQUIRED_SPACE: u32 = 30_000_000;

pub fn run(input: String) -> u32 {
    let commands = parse_commands(input);
    let root = Directory::from_commands(commands);

    let space_to_delete = REQUIRED_SPACE - (TOTAL_DISK_SPACE - root.borrow().size());
    if space_to_delete == 0 {
        return 0;
    }

    let mut stack = Vec::<Rc<RefCell<Directory>>>::new();
    stack.push(root);
    let mut out = u32::MAX;
    while let Some(node) = stack.pop() {
        let dir = node.borrow();
        let size = dir.size();
        if size > space_to_delete && size < out {
            out = size;
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
        24933642
    )
}
