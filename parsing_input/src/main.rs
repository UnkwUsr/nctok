struct DirEntry {
    name: String,
    number: usize,
    // if children is None - it is file, otherwise it is directory
    children: Option<Vec<DirEntry>>,
}

impl DirEntry {
    fn is_file(&self) -> bool {
        return self.children.is_none();
    }
    fn is_dir(&self) -> bool {
        return !self.is_file();
    }
}

use std::io;

fn main() {
    let root = Vec::<DirEntry>::new();
    let _ = io::stdin()
        .lines()
        .map(|x| {
            x.unwrap()
                .split_whitespace()
                .map(str::to_string)
                .collect::<Vec<String>>()
        })
        .map(|x| DirEntry {
            number: x.first().unwrap().parse().unwrap(),
            name: x.last().unwrap().to_string(),
            children: None,
        });

    println!("Hello, world!");
}
