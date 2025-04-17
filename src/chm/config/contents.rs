//! Module for managing dependencies
//!
//! Reads and stores files, converts docs to HTML
use crate::chm::inputs::md_load;
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
    #[allow(clippy::single_match_else)]
    pub fn add_file(&mut self, path: impl AsRef<Path>, contents: &[u8]) -> std::io::Result<()> {
        let src_path = path.as_ref();
        println!("Processing `{}`", src_path.display());

        let (file, dependencies) = match src_path.extension().and_then(OsStr::to_str) {
            Some("md") => md_load(src_path, contents)?,
            _ => {
                let file = File {
                    path: src_path.to_path_buf(),
                    contents: std::fs::read(path)?,
                };
                (file, vec![])
            }
        };

        for dependency in dependencies {
            let contents = std::fs::read(&dependency)?;
            self.add_file(dependency, &contents)?;
        }

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

    /// Attempts to convert the contents to a UTF-8 string.
    #[must_use]
    pub fn str_contents(&self) -> Option<&str> {
        if self.is_html() {
            std::str::from_utf8(&self.contents).ok()
        } else {
            None
        }
    }
}
