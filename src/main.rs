use clap::Parser;

mod arguments;
mod commands;
mod objects;
mod utils;

fn main() {
    let args = arguments::Args::parse();

    match args.command {
        arguments::Command::Init => commands::init::invoke(),
        arguments::Command::CatFile {
            pretty_print,
            object_hash,
        } => {
            if !pretty_print {
                todo!("not supported yet.")
            }

            commands::catfile::pretty_print(&object_hash).expect("failed to pretty print file")
        }
        arguments::Command::HashObject { write, file_path } => {
            commands::hashobject::invoke(&file_path, write)
        }
        arguments::Command::LsTree {
            name_only,
            tree_sha,
        } => commands::listtree::invoke(&tree_sha, name_only),
    }
}
