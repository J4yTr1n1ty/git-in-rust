#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;

use clap::Parser;
use clap::Subcommand;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::io::prelude::*;

#[derive(Parser, Debug)]
#[command(version = "0.0.1", about = "My own Implementation of (some of) the features of git", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[clap(about = "Initializes a git repository in the current directory")]
    Init,
    #[clap(about = "Prints out git objects")]
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,

        object_hash: String,
    },
    #[clap(about = "Hashes a given File")]
    HashObject {
        #[clap(short = 'w')]
        write: bool,

        file_path: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Init => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory")
        }
        Command::CatFile {
            pretty_print,
            object_hash,
        } => {
            if !pretty_print {
                todo!()
            }

            cat_file_pretty_print(&object_hash)
        }
        Command::HashObject { write, file_path } => {
            hash_object(&file_path, write);
        }
    }
}

fn cat_file_pretty_print(object_hash: &str) {
    // verify it's a sha1 hash
    if object_hash.len() != 40 {
        panic!("Invalid Object Hash");
    }

    // build the path
    let folder = &object_hash[..2].to_string();
    let rest_of_hash = &object_hash[2..].to_string();

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

    // read the file as bytes
    let mut file = File::open(&final_file_path).unwrap();
    let mut content = Vec::new();
    file.read_to_end(&mut content).unwrap();
    // decompress bytes
    let decompressed_string = decompress(content);
    // split and output
    let parts: Vec<&str> = decompressed_string.split('\0').collect();
    if parts.len() < 2 {
        panic!("Malformed object file.");
    }

    print!("{}", parts[1]);
}

fn hash_object(file_path: &str, write: bool) {
    // Get Content of File
    let path = Path::new(file_path);
    if !path.exists() {
        panic!("Given file path is invalid.")
    }
    let mut file = File::open(&path).expect("File unable to be opened.");
    let mut content = Vec::new();
    file.read_to_end(&mut content).unwrap();
    // Get sha1 hash of the file before Compression and output it
    let mut hasher = Sha1::new();

    hasher.update(&content);

    let file_hash = hasher.finalize();
    let hex_hash = hex::encode(file_hash);

    if write {
        write_object_file(&hex_hash, content);
    }

    println!("{}", &hex_hash);
}

fn write_object_file(hex_hash: &String, content: Vec<u8>) {
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
    let object_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(object_folder.join(&hex_hash[2..]));
    if !object_file.is_ok() {
        panic!("Failed to open file to write to.")
    }

    let _ = object_file.unwrap().write_all(&compressed_file_contents);
}

fn find_git_root() -> Option<String> {
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

fn decompress(content: Vec<u8>) -> String {
    let cursor = std::io::Cursor::new(content);

    // Now we can use GzDecoder with the Cursor
    let mut d = ZlibDecoder::new(cursor);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    return s;
}

fn compress(content: Vec<u8>) -> Vec<u8> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());

    // Convert the size of the content to a string
    let size_str = content.len().to_string();

    // Create a new vector that starts with "blob ", followed by the size, a null byte, and the content
    let mut to_compress = format!("blob {}\0", size_str).into_bytes();
    to_compress.extend_from_slice(&content);

    e.write_all(&to_compress).unwrap();

    return e.finish().unwrap();
}
