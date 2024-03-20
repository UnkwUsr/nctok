#[derive(Debug)]
enum HashEntry {
    Value(usize),
    Children(HashMap<String, HashEntry>),
}

impl HashEntry {
    fn new_empty_children() -> Self {
        HashEntry::Children(HashMap::<String, HashEntry>::new())
    }

    fn add_nested_items(&mut self, path: Vec<String>, name: String, value: usize) {
        match self {
            HashEntry::Value(_) => (),
            HashEntry::Children(childs) => {
                if path.is_empty() {
                    childs.insert(name, HashEntry::Value(value));
                    return;
                }

                let key = path[0].clone();
                let child = childs.entry(key).or_insert(HashEntry::new_empty_children());

                child.add_nested_items(path[1..].to_vec(), name, value);
            }
        }
    }
}

use std::{collections::HashMap, io};

fn main() {
    type ParentPath = Vec<String>;
    let mut root = HashEntry::new_empty_children();

    io::stdin().lines().for_each(|x| {
        let binding = x.unwrap();
        let mut y = binding.split_whitespace();

        let number: usize = y.next().unwrap().parse().unwrap();
        let name = y.next_back().unwrap().to_string();
        let parent_path: ParentPath = y.map(str::to_string).collect();

        root.add_nested_items(parent_path, name, number);
    });

    println!("{:#?}", root);
}
