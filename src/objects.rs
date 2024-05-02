pub const TYPE_REGULAR: i32 = 100644;
pub const TYPE_EXECUTABLE: i32 = 10755;
pub const TYPE_SYMLINK: i32 = 120000;

pub struct Tree {
    entries: Vec<TreeEntry>,
}

pub struct TreeEntry {
    r#type: i32,
    object_name: String,
    object_hash: String,
}
