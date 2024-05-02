#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::path::Path;

use clap::Parser;
use clap::Subcommand;
use flate2::read::GzDecoder;
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

            println!(
                "Path to file: {}",
                final_file_path.clone().into_os_string().to_string_lossy()
            );
            // read the file as bytes
            let content =
                fs::read_to_string(final_file_path).expect("Unable to read file contents.");
            // decompress bytes
            let decompressed_string = decompress(content);
            println!("Decompressed Input: {}", decompressed_string);
            // parse bytes
        }
    }
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

fn decompress(content: String) -> String {
    let mut d = GzDecoder::new(content.as_bytes());
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    return s;
}
