use std::path::{Path, PathBuf};

use slotmap::{new_key_type, SlotMap};

pub mod span;

#[derive(Debug, Clone)]
pub struct Source {
    pub path: PathBuf,
    pub content: String,
}
impl Source {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path).unwrap();
        Self {
            path: path.into(),
            content,
        }
    }
}

new_key_type! { pub struct SourceKey; }

pub type SourceMap = SlotMap<SourceKey, Source>;
