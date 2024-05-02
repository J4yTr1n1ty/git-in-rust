use clap::Parser;

mod arguments;
mod commands;
mod objects;
mod utils;

fn main() {
    let args = arguments::Args::parse();

    match args.command {
        arguments::Command::Init => commands::init_repository(),
        arguments::Command::CatFile {
            pretty_print,
            object_hash,
        } => {
            if !pretty_print {
                todo!()
            }

            commands::cat_file_pretty_print(&object_hash)
        }
        arguments::Command::HashObject { write, file_path } => {
            commands::hash_object(&file_path, write)
        }
        arguments::Command::LsTree {
            name_only,
            tree_sha,
        } => commands::list_tree(&tree_sha, name_only),
    }
}
