use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use indextree::{Arena, NodeEdge, NodeId};
use lazy_static::lazy_static;
use regex::Regex;
use std::convert::From;

#[derive(Debug, PartialEq, Clone)]
pub enum ShellLine {
    ChangeDir(String),
    ListDir,
    FileOutput(String, u64),
    DirOutput(String),
}

impl From<&str> for ShellLine {
    fn from(raw: &str) -> Self {
        lazy_static! {
            static ref CD_RE: Regex = Regex::new(r"^\$ cd (?P<dir>.+)$").unwrap();
            static ref LS_RE: Regex = Regex::new(r"^\$ ls$").unwrap();
            static ref FILE_RE: Regex = Regex::new(r"^(?P<size>\d+) (?P<file>.+)$").unwrap();
            static ref DIR_RE: Regex = Regex::new(r"^dir (?P<dir>.+)$").unwrap();
        }

        if CD_RE.is_match(raw) {
            let dirname = CD_RE.captures(raw).unwrap().name("dir").unwrap().as_str();
            Self::ChangeDir(dirname.to_string())
        } else if LS_RE.is_match(raw) {
            Self::ListDir
        } else if FILE_RE.is_match(raw) {
            let size = FILE_RE
                .captures(raw)
                .unwrap()
                .name("size")
                .unwrap()
                .as_str();
            let file = FILE_RE
                .captures(raw)
                .unwrap()
                .name("file")
                .unwrap()
                .as_str();
            Self::FileOutput(file.to_string(), size.parse::<u64>().unwrap())
        } else if DIR_RE.is_match(raw) {
            let dirname = DIR_RE.captures(raw).unwrap().name("dir").unwrap().as_str();
            Self::DirOutput(dirname.to_string())
        } else {
            unreachable!("unrecognized shell line format: {}", raw);
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum FSNode {
    File(String, u64),
    Dir(String),
}

struct FSTree {
    tree: Arena<FSNode>,
    current: NodeId,
    root: NodeId,
}

impl FSTree {
    fn new() -> Self {
        let mut tree = Arena::new();
        let root = tree.new_node(FSNode::Dir("/".to_string()));
        FSTree {
            tree: tree,
            current: root,
            root: root,
        }
    }

    fn size(&self, node_id: NodeId) -> u64 {
        match self.value_for_node(node_id) {
            FSNode::File(_, size) => size,
            FSNode::Dir(_) => node_id.children(&self.tree).map(|x| self.size(x)).sum(),
        }
    }

    fn dirs(&self) -> Vec<NodeId> {
        self.root
            .traverse(&self.tree)
            .filter_map(|edge| {
                if let NodeEdge::Start(node) = edge {
                    match self.value_for_node(node) {
                        FSNode::Dir(_) => Some(node),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    fn value_for_node(&self, node_id: NodeId) -> FSNode {
        self.tree
            .get(node_id)
            .map(|node| node.get().clone())
            .unwrap()
    }

    fn add_fs_node(&mut self, fs_node: FSNode) {
        let node = self.tree.new_node(fs_node);
        self.current.append(node, &mut self.tree);
    }

    fn change_dir(&mut self, dir: &str) {
        if dir == "/" {
            return; // noop
        }

        if dir == ".." {
            self.current = self.current.ancestors(&self.tree).skip(1).next().unwrap();
            return;
        }

        let node_value = FSNode::Dir(dir.to_string());
        let target = self
            .current
            .children(&self.tree)
            .find(|node_id| self.value_for_node(*node_id) == node_value)
            .unwrap();
        self.current = target;
    }
}

impl From<&[ShellLine]> for FSTree {
    fn from(input: &[ShellLine]) -> Self {
        let mut fs = FSTree::new();

        for line in input {
            match line {
                ShellLine::ChangeDir(dir) => fs.change_dir(dir),
                ShellLine::DirOutput(dir) => fs.add_fs_node(FSNode::Dir(dir.to_string())),
                ShellLine::FileOutput(file, size) => {
                    fs.add_fs_node(FSNode::File(file.to_string(), *size))
                }
                ShellLine::ListDir => {} // noop
            }
        }

        fs
    }
}

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<ShellLine> {
    input.lines().map(ShellLine::from).collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[ShellLine]) -> u64 {
    let fs = FSTree::from(input);
    fs.dirs()
        .into_iter()
        .filter_map(|dir| {
            let size = fs.size(dir);
            if size < 100000 {
                Some(size)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[ShellLine]) -> u64 {
    const MAX_CAPACITY: u64 = 70000000;
    const TARGET_CAPACITY: u64 = 30000000;

    let fs = FSTree::from(input);
    let used_size = fs.size(fs.root);

    fs.dirs()
        .into_iter()
        .filter_map(|dir| {
            let size = fs.size(dir);
            if MAX_CAPACITY - (used_size - size) >= TARGET_CAPACITY {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw_input() -> &'static str {
        r#"$ cd /
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
7214296 k"#
    }

    fn input() -> Vec<ShellLine> {
        vec![
            ShellLine::ChangeDir("/".to_string()),
            ShellLine::ListDir,
            ShellLine::DirOutput("a".to_string()),
            ShellLine::FileOutput("b.txt".to_string(), 14848514),
            ShellLine::FileOutput("c.dat".to_string(), 8504156),
            ShellLine::DirOutput("d".to_string()),
            ShellLine::ChangeDir("a".to_string()),
            ShellLine::ListDir,
            ShellLine::DirOutput("e".to_string()),
            ShellLine::FileOutput("f".to_string(), 29116),
            ShellLine::FileOutput("g".to_string(), 2557),
            ShellLine::FileOutput("h.lst".to_string(), 62596),
            ShellLine::ChangeDir("e".to_string()),
            ShellLine::ListDir,
            ShellLine::FileOutput("i".to_string(), 584),
            ShellLine::ChangeDir("..".to_string()),
            ShellLine::ChangeDir("..".to_string()),
            ShellLine::ChangeDir("d".to_string()),
            ShellLine::ListDir,
            ShellLine::FileOutput("j".to_string(), 4060174),
            ShellLine::FileOutput("d.log".to_string(), 8033020),
            ShellLine::FileOutput("d.ext".to_string(), 5626152),
            ShellLine::FileOutput("k".to_string(), 7214296),
        ]
    }

    #[test]
    fn test_fstree_size() {
        let input: &[ShellLine] = &input();
        let fs = FSTree::from(input);
        assert_eq!(fs.size(fs.root), 48381165);
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(raw_input()), input());
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&input()), 95437);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&input()), 24933642);
    }
}
