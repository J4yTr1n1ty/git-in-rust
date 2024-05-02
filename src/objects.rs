pub enum FileType {
    REGULAR = 100644,
    EXECUTABLE = 10755,
    SYMLINK = 120000,
}

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

pub struct TreeEntry {
    pub r#type: FileType,
    pub object_name: String,
    pub object_hash: String,
}
