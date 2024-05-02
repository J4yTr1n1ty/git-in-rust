use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

use sha1::{Digest, Sha1};

use crate::{
    objects::{FileType, Tree, TreeEntry},
    utils,
};

pub fn init_repository() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git directory")
}

pub fn cat_file_pretty_print(object_hash: &str) {
    let file_path = utils::get_object_path_by_hash(object_hash);

    let decompressed_string = utils::get_object_contents(file_path);

    // split and output
    let parts: Vec<&str> = decompressed_string.split('\0').collect();
    if parts.len() < 2 {
        panic!("Malformed object file.");
    }

    print!("{}", parts[1]);
}

pub fn hash_object(file_path: &str, write: bool) {
    // Get Content of File
    let path = Path::new(file_path);
    if !path.exists() {
        panic!("Given file path is invalid.")
    }
    let mut file = File::open(&path).expect("File unable to be opened.");
    let mut content = Vec::new();
    file.read_to_end(&mut content).unwrap();

    // Convert the size of the content to a string
    let size_str = content.len().to_string();

    // Create a new vector that starts with "blob ", followed by the size, a null byte, and the content
    let mut to_compress = format!("blob {}\0", size_str).into_bytes();
    to_compress.extend_from_slice(&content);

    // Get sha1 hash of the file before Compression and output it
    let mut hasher = Sha1::new();

    hasher.update(&to_compress);

    let file_hash = hasher.finalize();
    let hex_hash = hex::encode(file_hash);

    if write {
        utils::write_object_file(&hex_hash, to_compress);
    }

    println!("{}", &hex_hash);
}

pub fn list_tree(tree_hash: &str, names_only: bool) {
    let file_path = utils::get_object_path_by_hash(tree_hash);

    let decompressed_content = utils::get_object_contents_as_bytes(file_path);

    let mut tree: Tree = Tree::new();

    // Parse the header
    let header_end = decompressed_content
        .iter()
        .position(|&b| b == b'\0')
        .ok_or("Header not found")
        .unwrap();
    let (header, entries) = decompressed_content.split_at(header_end + 1);
    // let header_str = std::str::from_utf8(&header[..header_end]).unwrap();

    // Parse the entries
    let mut entries_iter = entries.split(|&b| b == b'\0');
    while let Some(entry) = entries_iter.next() {
        if entry.is_empty() {
            break;
        }
        let mode_end = entry
            .iter()
            .position(|&b| b == b' ')
            .ok_or("Mode not found")
            .unwrap();
        let (mode_bytes, entry_rest) = entry.split_at(mode_end);
        let mode = std::str::from_utf8(&mode_bytes).unwrap();
        let name_end = entry_rest
            .iter()
            .position(|&b| b == b'\0')
            .ok_or("Name not found")
            .unwrap();
        let (name_bytes, hash_bytes) = entry_rest.split_at(name_end);
        let name = std::str::from_utf8(&name_bytes[1..]).unwrap();
        let hash = hex::encode(hash_bytes);

        tree.entries.push(TreeEntry {
            mode: mode.to_string(),
            object_name: name.to_string(),
            object_hash: hash,
        })
    }

    for entry in tree.entries {
        if names_only {
            println!("{}", entry.object_name);
        } else {
            println!(
                "{} {}\t{}",
                entry.mode, entry.object_hash, entry.object_name
            );
        }
    }
}
