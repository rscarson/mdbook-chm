//! Contains utility functions used by the CHM modules
use std::path::{Path, PathBuf};

/// Path normalization functions to clean up the code
pub trait MakeAbsolute: AsRef<Path> {
    /// Append a relative path to the CWD
    /// 
    /// # Panics
    /// Panic if the CWD is missing
    fn make_absolute(&self) -> PathBuf {
        let path = self.as_ref();
        if path.is_absolute() {
            return path.to_path_buf();
        }
        std::env::current_dir()
            .unwrap_or_else(|_| panic!("Unable to get current directory"))
            .join(path)
    }

    /// Convert a path to a windows friendly string with only backslashes
    fn to_windows_path(&self) -> String {
        self.as_ref()
            .to_string_lossy()
            .to_string()
            .replace('/', "\\")
    }
}
impl MakeAbsolute for PathBuf {}
impl MakeAbsolute for Path {}

/// IO functions to clean up the code for writeouts
pub trait SafeWrite: AsRef<Path> {
    /// Create all parent directories needed for the a write.
    fn prepare_parent(&self) {
        if let Some(parent) = self.as_ref().parent() {
            std::fs::create_dir_all(parent).ok();
        }
    }

    /// Write to a file, after creating the parent directories
    /// 
    /// # Errors
    /// Will return an error if the file cannot be created or written to
    fn safe_write(&self, content: &[u8]) -> std::io::Result<()> {
        self.prepare_parent();
        let mut file = std::fs::File::create(self)?;
        std::io::Write::write_all(&mut file, content)?;
        Ok(())
    }

    /// Copy a file from one location to another, after creating the parent directories
    /// 
    /// # Errors
    /// Will return an error if the file cannot be created or written to, or the source
    /// cannot be read
    fn safe_copy(&self, source: impl AsRef<Path>) -> std::io::Result<()> {
        self.prepare_parent();
        let source = source.as_ref();
        if !source.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Source file not found: {}", source.display()),
            ));
        }
        std::fs::copy(source, self)?;
        Ok(())
    }
}
impl SafeWrite for PathBuf {}
impl SafeWrite for Path {}

/// Escape HTML special chars in a string
#[must_use]
pub fn escape_html(text: &str) -> String {
    let mut buffer = String::new();
    for c in text.chars() {
        match c {
            '&' => buffer.push_str("&amp;"),
            '<' => buffer.push_str("&lt;"),
            '>' => buffer.push_str("&gt;"),
            '"' => buffer.push_str("&quot;"),
            '\'' => buffer.push_str("&apos;"),
            _ => buffer.push(c),
        }
    }

    buffer
}

/// Locate a copy of the CHM compiler (hhc.exe)
/// 
/// Searches in this order:
/// - Current dir / path
/// - `C:\\Program Files (x86)\\HTML Help Workshop\\hhc.exe`
/// - Location stored in `CHM_COMPILER`
#[must_use]
pub fn find_compiler() -> Option<PathBuf> {
    //
    // First we search the current directory and PATH
    // The fastest way is just to try and invoke it
    if std::process::Command::new("hhc.exe")
        .arg("/?")
        .output()
        .is_ok()
    {
        return Some(PathBuf::from("hhc.exe"));
    }

    //
    // No luck so we search other common places
    let path = PathBuf::from("C:\\Program Files (x86)\\HTML Help Workshop\\hhc.exe");
    if path.exists() {
        return Some(path);
    }

    //
    // Otherwise try the CHM_COMPILER environment variable
    if let Ok(path) = std::env::var("CHM_COMPILER") {
        let path = PathBuf::from(path);
        if path.exists() {
            return Some(path);
        }
    }

    //
    // Unable to find the compiler
    None
}
