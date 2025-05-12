use std::{
    fmt::Display,
    fs::File,
    io::{self, BufReader, Read, Write},
    path::{Path, PathBuf},
};

/// Crate containing file system abstraction for crabreduce
pub struct Chunk {}

/// 64KB max chunk size... for now...
const CHUNK_SIZE: usize = 64 * 1024;

pub trait CrabFs {
    fn upload(&self, file: File) -> Result<(), FileSystemErr>;
    fn write_to(&self, path: PathBuf, buff: &[u8]) -> Result<(), FileSystemErr>;
    fn read(path: String) -> File;
}

pub struct LocalFileSystem {
    base: PathBuf,
}

pub struct FileSystem<FS: CrabFs> {
    _inner: FS,
}

pub enum FileSystemErr {
    DirectoryUploadError,
    SourceFileError,
    WriteError,
    CreateChunk,
    WriteChunk,
}

impl Display for FileSystemErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileSystemErr::SourceFileError => {
                write!(f, "Failed to read source file")
            }
            FileSystemErr::WriteError => {
                write!(f, "Failed to read source file")
            }
            FileSystemErr::CreateChunk => {
                write!(f, "Failed to create chunk")
            }
            FileSystemErr::WriteChunk => {
                write!(f, "Failed write to chunk")
            }
            FileSystemErr::DirectoryUploadError => {
                write!(f, "Directory upload not supported")
            }
        }
    }
}

impl LocalFileSystem {
    pub fn new(base: &str) -> Self {
        Self {
            base: PathBuf::from(base),
        }
    }
}

// TODO: Lock files...
impl CrabFs for LocalFileSystem {
    fn upload(&self, f: File) -> Result<(), FileSystemErr> {
        let metadata = f.metadata().map_err(|_| FileSystemErr::SourceFileError)?;

        // TODO: Handle directories
        if !metadata.is_file() {
            return Err(FileSystemErr::DirectoryUploadError);
        }

        let mut reader = BufReader::new(f);
        let mut buffer = [0u8; CHUNK_SIZE];
        let mut i = 0;
        loop {
            match reader.read(&mut buffer) {
                Ok(bytes) => {
                    if bytes == 0 {
                        return Ok(());
                    }

                    let chunk_file_name = self.base.join(format!("chunk-{}", i));
                    let mut chunk_file = File::create(chunk_file_name.clone())
                        .map_err(|_| FileSystemErr::CreateChunk)?;
                    chunk_file
                        .write_all(&buffer)
                        .map_err(|_| FileSystemErr::WriteChunk)?;
                    chunk_file.flush().map_err(|_| FileSystemErr::WriteChunk)?;
                    i += 1;
                }
                Err(_) => return Err(FileSystemErr::WriteError),
            }
        }
    }

    fn read(path: String) -> File {
        todo!()
    }

    fn write_to(&self, path: PathBuf, buff: &[u8]) -> Result<(), FileSystemErr> {
        todo!()
    }
}
