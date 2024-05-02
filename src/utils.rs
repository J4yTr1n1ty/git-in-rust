use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub fn get_object_path_by_hash(hash: &str) -> PathBuf {
    // verify it's a sha1 hash
    if hash.len() != 40 {
        panic!("Invalid Object Hash");
    }

    // build the path
    let folder = &hash[..2].to_string();
    let rest_of_hash = &hash[2..].to_string();

    let root_folder;
    match find_git_root() {
        Some(value) => root_folder = value,
        None => panic!("No git repository found in this or any parent directory."),
    }
    // Get the Path to the folder in which the object file lies. Also check if that
    // directory eixsts.
    let object_folder_str = format!("{root_folder}/.git/objects/{folder}");
    let object_folder = Path::new(&object_folder_str);
    if !object_folder.exists() {
        panic!("Object folder does not exist. Object Hash ist most likely wrong.")
    }

    let final_file_path = object_folder.join(rest_of_hash);

    return final_file_path;
}

pub fn get_object_contents(path: PathBuf) -> String {
    // read the file as bytes
    let mut file = File::open(&path).unwrap();
    let mut content = Vec::new();
    file.read_to_end(&mut content).unwrap();
    // decompress bytes
    let decompressed_string = decompress(content);

    return decompressed_string;
}

pub fn write_object_file(hex_hash: &String, content: Vec<u8>) {
    // Compress file contents
    let compressed_file_contents = compress(content);
    // Create folder with first two characters of the hash
    let root_folder;
    match find_git_root() {
        Some(value) => root_folder = value,
        None => panic!("No git repository found in this or any parent directory."),
    }
    let object_folder_str = format!("{root_folder}/.git/objects/{}", &hex_hash[0..2]);
    let object_folder = Path::new(&object_folder_str);
    if !object_folder.exists() {
        // Create directory if not exists (dir_all means also create parent directories)
        let create_result = fs::create_dir_all(object_folder);
        if !create_result.is_ok() {
            panic!("Unable to create object directory.")
        }
    }
    // Write compressed contents to file with name of the remaining characters of the hash
    let mut object_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(object_folder.join(&hex_hash[2..]))
        .unwrap_or_else(|e| {
            panic!("Failed to open file to write to. Hash: {hex_hash}, Folder: {object_folder_str}, Error: {e}")
        });

    object_file
        .write_all(&compressed_file_contents)
        .expect("Writing to File failed.");
}

pub fn find_git_root() -> Option<String> {
    let mut current_dir = env::current_dir().expect("Failed to get current directory");

    loop {
        let git_dir = current_dir.join(".git");
        if git_dir.exists() {
            return Some(
                current_dir
                    .to_str()
                    .expect("Failed to convert path to string")
                    .to_string(),
            );
        }

        match current_dir.parent() {
            Some(parent) => current_dir = parent.to_path_buf(),
            None => return None, // Reached the root of the filesystem without finding a .git directory
        }
    }
}

pub fn decompress(content: Vec<u8>) -> String {
    let cursor = std::io::Cursor::new(content);

    // Now we can use GzDecoder with the Cursor
    let mut d = ZlibDecoder::new(cursor);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    return s;
}

pub fn compress(content: Vec<u8>) -> Vec<u8> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());

    e.write_all(&content).unwrap();

    return e.finish().unwrap();
}
