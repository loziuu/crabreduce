use std::{env, fs::File, path::PathBuf};

use crab_fs::{FileSystem, WriteFile};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Basic usage: {} <file-path>", args[0]);
    }

    let mut crab_fs = FileSystem::new("/home/loziuu/crab");

    let buf = PathBuf::from(args[1].clone());
    let file_name = match buf.file_name() {
        Some(name) => name.to_str().unwrap(),
        None => panic!("Not a file!"),
    };

    let mut to = None;
    if args.len() > 2 {
        println!("Uploading to directory {}", args[2]);
        let path = args[2].clone();
        to = Some(PathBuf::from(path));
    }

    if let Ok(f) = File::open(PathBuf::from(args[1].clone())) {
        let cmd = match to {
            Some(path) => WriteFile::to_dir(file_name, f, path),
            None => WriteFile::to_root(file_name, f),
        };

        match crab_fs.write_file(cmd) {
            Ok(_) => println!("Saved!"),
            Err(e) => {
                eprintln!("Failed to upload file. {}", e);
            }
        }
    } else {
        eprintln!("Error: File {} was not found.", args[1]);
    }
}
