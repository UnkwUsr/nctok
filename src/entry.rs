use std::collections::HashMap;

#[derive(Debug)]
pub struct Entry {
    pub size: usize,
    pub children: Option<HashMap<String, Entry>>,
}
