use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Entry {
    pub size: usize,
    pub children: Option<BTreeMap<String, Entry>>,
}
