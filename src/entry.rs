use std::collections::HashMap;

#[derive(Debug)]
pub enum Entry {
    Value(usize),
    Children(HashMap<String, Entry>),
}

impl Entry {
    pub fn items(&self) -> Option<&HashMap<String, Entry>> {
        match self {
            Entry::Value(_) => None,
            Entry::Children(childs) => Some(childs),
        }
    }
}
