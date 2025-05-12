use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, Read, Write},
    path::PathBuf,
};

use metadata::Metadata;

pub(crate) mod files;
mod metadata;

/// Crate containing file system abstraction for crabreduce
pub struct Chunk {
    data: [u8; CHUNK_SIZE],
}

/// 64KB max chunk size... for now...
const CHUNK_SIZE: usize = 64 * 1024;

pub trait CrabFs {
    fn upload(
        &self,
        file_name: &str,
        file: File,
        path: PathBuf,
    ) -> Result<UploadResult, FileSystemErr>;
}

pub struct UploadResult {
    path: PathBuf,
    chunks: usize,
}

// Metadata?
pub struct LocalFileSystem {
    base: PathBuf,
    metadata: Metadata,
}

pub struct FileSystem<FS: CrabFs> {
    inner: FS,
    metadata: Metadata,
}

impl FileSystem<LocalFileSystem> {
    pub fn new(base: &str) -> Self {
        Self {
            inner: LocalFileSystem::new(base),
            metadata: Metadata::new(PathBuf::from(base)),
        }
    }

    pub fn write_file(&mut self, cmd: WriteFile) -> Result<(), FileSystemErr> {
        // TODO: Lock in metadata
        let result = self.inner.upload(
            cmd.name,
            cmd.file,
            cmd.path
                .map(|it| self.inner.base.join(it))
                .unwrap_or(self.inner.base.clone()),
        )?;
        self.metadata.insert_file(result.into());
        Ok(())
    }
}

pub struct WriteFile<'a> {
    pub name: &'a str,
    pub file: File,
    pub path: Option<PathBuf>,
}

impl<'a> WriteFile<'a> {
    pub fn to_root(name: &'a str, file: File) -> Self {
        Self {
            name,
            file,
            path: None,
        }
    }

    pub fn to_dir(name: &'a str, file: File, path: PathBuf) -> Self {
        Self {
            name,
            file,
            path: Some(path),
        }
    }
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
            metadata: Metadata::new(PathBuf::from(base)),
        }
    }
}

// TODO: Lock files...
impl CrabFs for LocalFileSystem {
    fn upload(&self, name: &str, f: File, to: PathBuf) -> Result<UploadResult, FileSystemErr> {
        println!("uploading file {}", name);
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
                        return Ok(UploadResult {
                            path: to.join(name),
                            chunks: i,
                        });
                    }

                    let chunk_file_name = to.join(format!("{}-{}", name, i));

                    std::fs::create_dir_all(&to).map_err(|_| {
                        eprintln!("Error: {}", FileSystemErr::DirectoryUploadError);
                        FileSystemErr::DirectoryUploadError
                    })?;

                    let mut chunk_file = File::create(chunk_file_name.clone()).map_err(|e| {
                        eprintln!("Error: {}", e);
                        FileSystemErr::CreateChunk
                    })?;
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
}
