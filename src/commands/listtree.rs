use std::{
    ffi::CStr,
    io::{BufRead, Read},
};

use anyhow::Context;

use crate::objects::{self, Kind};

struct Tree {
    entries: Vec<TreeEntry>,
}

struct TreeEntry {
    mode: u64,
    name: String,
    hash: String,
}

pub fn invoke(tree_hash: &str, names_only: bool) -> anyhow::Result<()> {
    let mut object = objects::Object::read(tree_hash).expect("Unable to read Object.");

    if object.kind != Kind::Tree {
        panic!("Unable to list tree for non Tree Object.")
    }

    let mut tree = Tree {
        entries: Vec::new(),
    };

    // Format:
    // <mode> <name>\0<20_byte_sha>
    let mut read_bytes: u64 = 0;
    while read_bytes < object.expected_size {
        let mut buf = Vec::new();
        object
            .reader
            .read_until(0, &mut buf)
            .context("read mode and name")
            .unwrap();
        read_bytes += buf.len() as u64;
        let beginning =
            CStr::from_bytes_with_nul(&buf).expect("Unable to parse beginning part of tree entry.");
        let beginning = beginning.to_str().unwrap();
        let Some((mode, name)) = beginning.split_once(' ') else {
            anyhow::bail!("Unable to parse mode and/or name in tree object.");
        };
        let mode = mode.parse::<u64>().unwrap();
        let name = name.to_string();
        let mut sha_buf = Vec::new();
        sha_buf.resize(20, 0);
        object.reader.read_exact(&mut sha_buf).unwrap();
        read_bytes += sha_buf.len() as u64;
        let hash = hex::encode(sha_buf);

        tree.entries.push(TreeEntry { mode, name, hash })
    }

    for entry in tree.entries {
        if names_only {
            println!("{}", entry.name);
        } else {
            println!(
                "{:0width$} {}\t{}",
                entry.mode,
                entry.hash,
                entry.name,
                width = 6
            );
        }
    }

    Ok(())
}
