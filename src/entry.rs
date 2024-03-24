use indexmap::IndexMap;

#[derive(Debug)]
pub struct Entry {
    pub size: usize,
    pub children: Option<IndexMap<String, Entry>>,
}
