#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(version = "0.0.1", about = "My own Implementation of (some of) the features of git", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init,
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
    }
}
