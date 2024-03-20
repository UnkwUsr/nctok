use crate::entry::Entry;
use std::{collections::HashMap, io};

type EntryPath = Vec<String>;

impl Entry {
    fn new_empty_children() -> Self {
        Entry::Children(HashMap::<String, Entry>::new())
    }

    fn add(&mut self, path: EntryPath, name: String, value: usize) {
        match self {
            Entry::Value(_) => (),
            Entry::Children(childs) => {
                if path.is_empty() {
                    childs.insert(name, Entry::Value(value));
                    return;
                }

                let key = path[0].clone();
                let child = childs.entry(key).or_insert(Entry::new_empty_children());

                child.add(path[1..].to_vec(), name, value);
            }
        }
    }
}

pub fn parse_stdin() -> Entry {
    let mut root = Entry::new_empty_children();

    io::stdin().lines().for_each(|x| {
        let binding = x.unwrap();
        let mut y = binding.split_whitespace();

        let number: usize = y.next().unwrap().parse().unwrap();
        let name = y.next_back().unwrap().to_string();
        let parent_path: EntryPath = y.map(str::to_string).collect();

        root.add(parent_path, name, number);
    });

    root
}
