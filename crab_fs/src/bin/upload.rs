use std::{env, fs::File, path::PathBuf};

use crab_fs::{CrabFs, LocalFileSystem};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Basic usage: {} <file-path>", args[0]);
    }

    let crab_fs = LocalFileSystem::new("/home/loziuu/crab/");
    if let Ok(f) = File::open(PathBuf::from(args[1].clone())) {
        match crab_fs.upload(f) {
            Ok(_) => println!("Saved!"),
            Err(e) => {
                eprintln!("Failed to upload file. {}", e);
            }
        }
    } else {
        eprintln!("Error: File {} was not found.", args[1]);
    }
}
