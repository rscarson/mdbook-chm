use std::path::{Path, PathBuf};

pub trait MakeAbsolute: AsRef<Path> {
    fn make_absolute(&self) -> PathBuf {
        let path = self.as_ref();
        if path.is_absolute() {
            return path.to_path_buf();
        }
        std::env::current_dir()
            .unwrap_or_else(|_| panic!("Unable to get current directory"))
            .join(path)
    }

    fn to_windows_path(&self) -> String {
        self.as_ref()
            .to_string_lossy()
            .to_string()
            .replace("/", "\\")
    }
}
impl MakeAbsolute for PathBuf {}
impl MakeAbsolute for Path {}

pub trait SafeWrite: AsRef<Path> {
    fn prepare_parent(&self) {
        if let Some(parent) = self.as_ref().parent() {
            std::fs::create_dir_all(parent).ok();
        }
    }

    fn safe_write(&self, content: &[u8]) -> std::io::Result<()> {
        self.prepare_parent();
        let mut file = std::fs::File::create(self)?;
        std::io::Write::write_all(&mut file, content)?;
        Ok(())
    }

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
