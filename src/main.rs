use std::fs;

use clap::Parser;
use clap::Subcommand;

mod commands;
mod objects;
mod utils;

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
    LsTree {
        #[clap(long = "name-only")]
        name_only: bool,

        tree_sha: String,
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

            commands::cat_file_pretty_print(&object_hash)
        }
        Command::HashObject { write, file_path } => {
            commands::hash_object(&file_path, write);
        }
        Command::LsTree {
            name_only,
            tree_sha,
        } => {
            if !name_only {
                todo!()
            }

            let file_path = utils::get_object_folder_by_hash(&tree_sha);
        }
    }
}
