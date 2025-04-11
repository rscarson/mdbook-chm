//! If you need a copy of the CHM compiler, you can find one at:  
//! <https://github.com/EWSoftware/SHFB/blob/master/ThirdPartyTools/htmlhelp.exe>

mod chm;
pub use chm::{ChmBuilder, ChmTopicBuilder, language::ChmLanguage};

mod mdbook;
pub use mdbook::{context_to_chm, get_context};

pub trait MakeAbsolute {
    fn make_absolute(&self) -> std::path::PathBuf;
}
impl MakeAbsolute for std::path::PathBuf {
    fn make_absolute(&self) -> std::path::PathBuf {
        if self.is_absolute() {
            return self.clone();
        }
        std::env::current_dir()
            .unwrap_or_else(|_| panic!("Unable to get current directory"))
            .join(self)
    }
}

pub trait SafeWrite: AsRef<std::path::Path> {
    fn prepare_parent(&self) {
        if let Some(parent) = self.as_ref().parent() {
            std::fs::create_dir_all(parent).ok();
        }
    }

    fn safe_write(&self, content: impl ToString) -> std::io::Result<()> {
        self.prepare_parent();
        let mut file = std::fs::File::create(self)?;
        std::io::Write::write_all(&mut file, content.to_string().as_bytes())?;
        Ok(())
    }

    fn safe_copy(&self, source: impl AsRef<std::path::Path>) -> std::io::Result<()> {
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
impl SafeWrite for std::path::PathBuf {}
impl SafeWrite for std::path::Path {}
