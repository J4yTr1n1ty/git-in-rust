pub enum FileType {
    REGULAR = 100644,
    EXECUTABLE = 10755,
    SYMLINK = 120000,
}

#[derive(Debug)]
pub struct Tree {
    pub entries: Vec<TreeEntry>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            entries: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct TreeEntry {
    pub mode: String,
    pub object_name: String,
    pub object_hash: String,
}
