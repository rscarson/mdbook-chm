//! Module for managing dependencies
//! 
//! Reads and stores files, converts docs to HTML
use crate::chm::{inputs::md_load, utilities::MakeAbsolute};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

/// All the files included in the CHM file.
#[derive(Debug, Clone)]
pub struct IncludedFiles {
    /// The list of files found.  
    /// The last one is the original file included
    pub files: Vec<File>,
}
impl Default for IncludedFiles {
    fn default() -> Self {
        Self::new()
    }
}
impl IncludedFiles {
    /// Creates a new `IncludedFiles` instance with the given source root and optional body regex.
    #[must_use]
    pub fn new() -> Self {
        Self { files: vec![] }
    }

    /// Adds a file to the list of included files, and processes it if it is an HTML file.
    /// 
    /// # Errors
    /// Will pass down IO errors
    pub fn add_file(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let src_path = path.as_ref();
        println!("Processing `{}`", src_path.make_absolute().display());

        //
        // Make sure the path is valid
        if src_path.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Path is a directory: {}", src_path.display()),
            ));
        }

        let file = match src_path.extension().and_then(OsStr::to_str) {
            Some("md") => md_load(src_path)?,
            _ => File {
                path: src_path.to_path_buf(),
                contents: std::fs::read(path)?,
            }
        };
        self.files.push(file);

        Ok(())
    }

    /// Adds a file to the list of included files, but does not process it.
    pub fn append(&mut self, other: Self) {
        self.files.extend(other.files);
    }
}

/// A file included in the CHM file.
#[derive(Debug, Clone)]
pub struct File {
    /// Target path after writing
    pub path: PathBuf,

    /// File contents
    pub contents: Vec<u8>,
}
impl File {
    /// Returns true if this file is an HTML document
    pub fn is_html(&self) -> bool {
        self.path.extension().and_then(OsStr::to_str) == Some("html")
    }
}
