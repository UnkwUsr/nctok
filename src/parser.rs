use crate::entry::Entry;
use indexmap::IndexMap;
use std::io;

type EntryPath = Vec<String>;

impl Entry {
    fn add(&mut self, path: EntryPath, name: String, value: usize) {
        self.size += value;

        let childs = self
            .children
            .get_or_insert(IndexMap::<String, Entry>::new());

        if path.is_empty() {
            childs.insert(
                name,
                Entry {
                    size: value,
                    children: None,
                },
            );
            // TODO: future optimization (probably): sort once after everything added
            childs.sort_unstable_by(|_ak, av, _bk, bv| bv.size.cmp(&av.size));
            return;
        }

        let key = path[0].clone();
        let child = childs.entry(key).or_insert(Entry {
            size: 0,
            children: None,
        });

        child.add(path[1..].to_vec(), name, value);
        childs.sort_unstable_by(|_ak, av, _bk, bv| bv.size.cmp(&av.size));
    }
}

pub fn parse_stdin() -> Entry {
    let mut root = Entry {
        size: 0,
        children: None,
    };

    io::stdin().lines().for_each(|x| {
        if let Some((number, suffix)) = x.unwrap().split_once(" ") {
            let number: usize = number.parse().unwrap();

            let mut path = suffix.split('/');
            let name = path.next_back().unwrap().to_string();
            let parent_path: EntryPath = path.map(str::to_string).collect();

            root.add(parent_path, name, number);
        }
    });

    root
}
