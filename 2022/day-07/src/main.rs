use std::{collections::HashMap, fs::read_to_string, iter::once};

#[derive(Debug)]
enum Item {
    Directory(Directory),
    File(File),
}

impl Item {
    fn file(size: u32) -> Self {
        Item::File(File { size })
    }

    fn directory() -> Self {
        Item::Directory(Directory::default())
    }

    fn size(&self) -> u32 {
        match self {
            Item::Directory(directory) => directory.size(),
            Item::File(file) => file.size,
        }
    }

    fn at_mut(&mut self, path: &[String]) -> Option<&mut Self> {
        let mut current = self;
        for element in path {
            current = match current {
                Item::Directory(directory) => directory.children.get_mut(element)?,
                Item::File(_) => return None,
            }
        }

        Some(current)
    }
}

#[derive(Debug, Default)]
struct Directory {
    children: HashMap<String, Item>,
}

impl Directory {
    fn size(&self) -> u32 {
        self.children
            .iter()
            .map(|(_name, child)| child.size())
            .sum()
    }
}

#[derive(Debug)]
struct File {
    size: u32,
}

fn generate_tree(input: &str) -> Item {
    let mut root = Item::directory();

    let mut current_path = Vec::new();

    for line in input.lines() {
        let mut words = line.split_whitespace();
        match (words.next().unwrap(), words.next().unwrap(), words.next()) {
            ("$", "cd", Some("/")) => {
                current_path = Vec::new();
            }
            ("$", "cd", Some("..")) => {
                current_path.pop();
            }
            ("$", "cd", Some(name)) => {
                let current_directory = match root.at_mut(&current_path) {
                    Some(Item::Directory(directory)) => directory,
                    Some(Item::File(_)) => panic!("Tried to cd into a file"),
                    None => panic!("current directory does not exist!"),
                };
                current_directory
                    .children
                    .entry(name.to_string())
                    .or_insert_with(Item::directory);
                current_path.push(name.to_string());
            }
            ("$", "ls", None) => {}
            ("dir", name, None) => {
                let current_directory = match root.at_mut(&current_path) {
                    Some(Item::Directory(directory)) => directory,
                    Some(Item::File(_)) => panic!("Tried to cd into a file"),
                    None => panic!("current directory does not exist!"),
                };
                current_directory
                    .children
                    .entry(name.to_string())
                    .or_insert_with(Item::directory);
            }
            (size, name, None) => {
                let current_directory = match root.at_mut(&current_path) {
                    Some(Item::Directory(directory)) => directory,
                    Some(Item::File(_)) => panic!("Tried to cd into a file"),
                    None => panic!("current directory does not exist!"),
                };
                let size = size.parse().expect("file size not an integer: {size}");
                current_directory
                    .children
                    .entry(name.to_string())
                    .or_insert_with(|| Item::file(size));
            }
            words => panic!("Unknown command: {words:?}"),
        }
    }

    root
}

fn task_1(tree: &Item) -> u32 {
    match tree {
        Item::Directory(directory) => {
            let total = directory
                .children
                .iter()
                .map(|(_name, item)| task_1(item))
                .sum();
            let size = tree.size();
            if size < 100_000 {
                total + size
            } else {
                total
            }
        }
        Item::File(_) => 0,
    }
}

fn task_2(tree: &Item, minimum_size: u32) -> Option<u32> {
    match tree {
        Item::Directory(directory) => directory
            .children
            .iter()
            .filter_map(|(_name, item)| task_2(item, minimum_size))
            .chain(once(tree.size()))
            .filter(|size| *size > minimum_size)
            .min(),
        Item::File(_) => None,
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let tree = generate_tree(&input);
    println!("{:#?}", tree);
    println!("Task 1: {}", task_1(&tree));
    let total_space = 70000000;
    let used_space = tree.size();
    let free_space = total_space - used_space;
    let required_additional_space = 30000000 - free_space;
    println!(
        "Task 2: {}",
        task_2(&tree, required_additional_space).expect("no suitable directory to delete found")
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_task_1() {
        let input = read_to_string("example").unwrap();
        let tree = generate_tree(&input);

        assert_eq!(tree.size(), 48381165);
    }

    #[test]
    fn example_used_space() {
        let input = read_to_string("example").unwrap();
        let tree = generate_tree(&input);

        assert_eq!(tree.size(), 48381165);
    }
}
