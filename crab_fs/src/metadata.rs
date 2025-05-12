use std::path::PathBuf;

use crate::{UploadResult, files::Files};

pub struct Metadata {
    files: Files,
}

impl Metadata {
    pub fn new(base: PathBuf) -> Metadata {
        Metadata {
            files: Files::new(base),
        }
    }

    // TODO: Add lock handle?
    pub fn insert_file(&mut self, metadata: FileMetadata) {
        self.files.put(metadata.path, metadata.chunks);
    }
}

pub(crate) struct FileMetadata {
    path: PathBuf,
    chunks: usize,
}

impl From<UploadResult> for FileMetadata {
    fn from(value: UploadResult) -> Self {
        Self {
            path: value.path,
            chunks: value.chunks,
        }
    }
}
