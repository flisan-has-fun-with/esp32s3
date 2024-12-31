use std::path::{Path, PathBuf};

pub trait PathExt {
    fn coerce_to_string(&self) -> String;
}

impl PathExt for PathBuf {
    fn coerce_to_string(&self) -> String {
        self.as_path().coerce_to_string()
    }
}

impl PathExt for Path {
    fn coerce_to_string(&self) -> String {
        self.to_str().unwrap().to_string()
    }
}

