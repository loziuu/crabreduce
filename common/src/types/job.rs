use std::ops::Deref;

use super::kv::{Key, KeyValue, Value};

pub trait Job {
    fn map(&self, kv: KeyValue) -> Vec<KeyValue>;
    fn reduce(&self, k: Key, v: Vec<Value>) -> KeyValue;
}

pub struct FileName(String);

pub enum FileNameError {
    InvalidName(&'static str),
}

impl FileName {
    pub fn new(file_name: String) -> Result<FileName, FileNameError> {
        // Validate filename

        if file_name.is_empty() {
            return Err(FileNameError::InvalidName("Empty file name"));
        }

        Ok(FileName(file_name))
    }
}

pub struct Task {
    file_name: String,
}

impl Deref for FileName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<String> for FileName {
    type Error = FileNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        FileName::new(value)
    }
}
