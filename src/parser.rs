use crate::entry::Entry;
use std::{collections::BTreeMap, io};

type EntryPath = Vec<String>;

impl Entry {
    fn add(&mut self, path: EntryPath, name: String, value: usize) {
        self.size += value;

        let childs = self
            .children
            .get_or_insert(BTreeMap::<String, Entry>::new());

        if path.is_empty() {
            childs.insert(
                name,
                Entry {
                    size: value,
                    children: None,
                },
            );
            return;
        }

        let key = path[0].clone();
        let child = childs.entry(key).or_insert(Entry {
            size: 0,
            children: None,
        });

        child.add(path[1..].to_vec(), name, value);
    }
}

pub fn parse_stdin() -> Entry {
    let mut root = Entry {
        size: 0,
        children: None,
    };

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
