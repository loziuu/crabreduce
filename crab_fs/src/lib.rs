use std::{fs::File, path::Path};

/// Crate containing file system abstraction for crabreduce
pub struct Chunk {}

pub trait CrabFs {
    fn write(file: File);
    fn read(path: String) -> File;
}

pub struct LocalFileSystem {
    _base: Path,
}

pub struct FileSystem<FS: CrabFs> {
    _inner: FS,
}

// TODO: Lock files...
impl CrabFs for LocalFileSystem {
    fn write(_: File) {
        // TODO: Split to chunks...
        todo!()
    }

    /// Stream chunks?
    fn read(_: String) -> File {
        // TODO: let p = Path::from(path);
        todo!()
    }
}
