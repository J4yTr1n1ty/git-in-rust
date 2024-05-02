use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(version = "0.0.1", about = "My own Implementation of (some of) the features of git", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
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
