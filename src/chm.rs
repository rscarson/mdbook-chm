use crate::{MakeAbsolute, SafeWrite};
use std::path::{Path, PathBuf};

pub mod hhc;
pub mod hhk;
pub mod hhp;
pub mod language;

#[derive(Debug, Clone)]
pub struct Chm {
    pub project: hhp::ChmProject,
    pub contents: hhc::ChmContents,
    pub index: hhk::ChmIndex,
}

pub struct ChmBuilder {
    chm: Chm,
    project_path: PathBuf,
}
impl ChmBuilder {
    pub fn new(
        title: impl AsRef<str>,
        language: language::ChmLanguage,
        output_path: impl AsRef<Path>,
    ) -> Self {
        let output_path = output_path.as_ref().with_extension("chm");
        let project_path = output_path.with_extension("hhp").make_absolute();
        let contents_path = output_path.with_extension("hhc").make_absolute();
        let index_path = output_path.with_extension("hhk").make_absolute();

        let project = hhp::ChmProject {
            title: escape_html(title.as_ref()),
            language,
            output_path: output_path.to_string_lossy().to_string(),
            index_path: index_path.to_string_lossy().to_string(),
            contents_path: contents_path.to_string_lossy().to_string(),
            files: vec![],
        };
        let contents = hhc::ChmContents(vec![]);
        let index = hhk::ChmIndex(vec![]);

        let chm = Chm {
            project,
            contents,
            index,
        };

        Self { chm, project_path }
    }

    pub fn with_contents(mut self, topic: ChmTopicBuilder) -> Self {
        let flat_map = topic.0.clone().flatten();
        let filenames = flat_map.iter().map(|e| e.file.clone()).collect::<Vec<_>>();
        let index = flat_map.into_iter().map(|e| hhk::ChmIndexEntry {
            keyword: e.title,
            file: e.file,
        });

        self.chm.contents.0.push(topic.0);
        self.chm.project.files.extend(filenames);
        self.chm.index.0.extend(index);

        self
    }

    /// Writes the CHM project component files to the specified output paths.  
    /// Does NOT compile the CHM file.
    pub fn write(&self) -> std::io::Result<()> {
        let project_path = &self.project_path;
        let project = self.chm.project.to_string();
        project_path.safe_write(&project)?;

        let contents_path = PathBuf::from(self.chm.project.contents_path.clone());
        let contents = self.chm.contents.to_string();
        contents_path.safe_write(&contents)?;

        let index_path = PathBuf::from(self.chm.project.index_path.clone());
        index_path.safe_write(&self.chm.index)?;

        Ok(())
    }

    /// Writes the CHM project component files to the specified output paths and compiles the CHM file.
    pub fn compile(self) -> std::io::Result<()> {
        self.write()?;
        let compiler_path = find_compiler()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Compiler not found. Please install HTML Help Workshop, or provide the CHM_COMPILER variable"))?;
        let mut command = std::process::Command::new(compiler_path);
        command
            .arg(self.project_path)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit());

        command.spawn()?.wait()?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ChmTopicBuilder(hhc::ChmContentsEntry);
impl ChmTopicBuilder {
    pub fn new(title: impl AsRef<str>, file: impl AsRef<str>) -> Self {
        let file = Path::new(file.as_ref()).to_path_buf().make_absolute();
        let file = file.to_string_lossy().replace('/', "\\");

        let topic = hhc::ChmContentsEntry {
            title: escape_html(title.as_ref()),
            file: escape_html(&file),
            children: vec![],
        };
        Self(topic)
    }

    pub fn with_child(&mut self, child: ChmTopicBuilder) -> &mut Self {
        self.0.children.push(child.0);
        self
    }
}

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
