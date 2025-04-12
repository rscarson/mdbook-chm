use hhc::ChmContentsEntry;
use hhk::{ChmIndex, ChmIndexEntry};

use super::utilities::{MakeAbsolute, SafeWrite, escape_html, find_compiler};
use std::path::{Path, PathBuf};

pub mod contents;

pub mod hhc;
pub mod hhk;
pub mod hhp;

mod language;
pub use language::ChmLanguage;

/// Allows for simplified creation of a CHM project and dependencies
/// 
/// Manages file conversions, dependencies, encoding issues, and write-out to the working dir
#[derive(Debug, Clone)]
pub struct ChmBuilder {
    project: hhp::ChmProject,
    contents: hhc::ChmContents,
    project_path: PathBuf,
    working_dir: PathBuf,
}
impl ChmBuilder {
    /// Create a new CHM builder
    /// 
    /// Requires the book title, requested language code, and path to put the output files
    pub fn new(
        title: impl AsRef<str>,
        language: language::ChmLanguage,
        output_path: impl AsRef<Path>,
    ) -> Self {
        let output_path = output_path.as_ref().with_extension("chm");
        let output_dir = output_path.parent().unwrap_or_else(|| Path::new("."));
        let working_dir = output_dir.join("src").make_absolute();

        let project_path = working_dir.join("project.hpp");
        let contents_path = working_dir.join("contents.hhc");
        let index_path = working_dir.join("index.hhk");

        let project = hhp::ChmProject {
            title: escape_html(title.as_ref()),
            language,
            output_path: output_path.to_windows_path(),
            index_path: index_path.to_windows_path(),
            contents_path: contents_path.to_windows_path(),
            default_file: String::new(),
        };
        let contents = hhc::ChmContents(vec![]);

        Self {
            project,
            contents,
            project_path,
            working_dir,
        }
    }

    /// Include a topic in the CHM file.  
    /// Topics can nest deeply.
    pub fn with_contents(&mut self, topic: ChmTopicBuilder) -> &mut Self {
        if self.project.default_file.is_empty() {
            if let Some(last) = topic.0.files.last() {
                self.project.default_file = last.path.to_windows_path();
            }
        }

        self.contents.0.push(topic.0);
        self
    }

    /// Writes the CHM project component files to the specified output paths.  
    /// Does NOT compile the CHM file.
    /// 
    /// # Errors
    /// Can return an error if output writes fail
    pub fn write(&self) -> std::io::Result<()> {
        //
        // Write project file
        let project_path = &self.project_path;
        let project = self.project.to_string();
        println!("Writing {}", project_path.display());
        project_path.safe_write(project.as_bytes())?;

        //
        // Write TOC
        let contents_path = PathBuf::from(self.project.contents_path.clone());
        let contents = self.contents.to_string();
        println!("Writing {}", contents_path.display());
        contents_path.safe_write(contents.as_bytes())?;

        //
        // Flatten TOC to finish building the project files
        let flat_map = self.contents.clone().flatten();
        let index = ChmIndex(
            flat_map
                .iter()
                .map(|entry| ChmIndexEntry {
                    keyword: escape_html(&entry.title),
                    file: escape_html(&entry.file),
                })
                .collect(),
        );
        let files = flat_map
            .iter()
            .flat_map(|entry| entry.files.clone())
            .collect::<Vec<_>>();

        //
        // Write index
        let index_path = PathBuf::from(self.project.index_path.clone());
        let index = index.to_string();
        println!("Writing {}", index_path.display());
        index_path.safe_write(index.as_bytes())?;

        //
        // Write dependencies
        for file in &files {
            let target_path = self.working_dir.join(&file.path);
            println!("Writing {}", target_path.display());
            target_path.safe_write(&file.contents)?;
        }

        Ok(())
    }

    /// Writes the CHM project component files to the specified output paths and compiles the CHM file.
    /// 
    /// # Errors
    /// Can return an error if the compiler cannot be located, or on IO errors, or if compilation fails
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

/// Create a CHM topic (chapter) based on an input file.
/// 
/// Will pull any dependencies (images, stylesheets, etc),
/// 
/// And convert compatible files to HTML. See [`crate::chm::inputs`] for the list of formats supported
#[derive(Debug, Clone)]
pub struct ChmTopicBuilder(ChmContentsEntry);
impl ChmTopicBuilder {
    /// Build a topic based on a file
    /// 
    /// # Errors
    /// Will return an error on IO failures, or if the file references dead images
    pub fn new(title: &impl ToString, file: impl AsRef<Path>) -> std::io::Result<Self> {
        let topic = ChmContentsEntry::new(title, file)?;
        Ok(Self(topic))
    }

    /// Add a subtopic to this topic
    /// 
    /// See [`ChmTopicBuilder::new`]
    pub fn with_child(&mut self, child: ChmTopicBuilder) -> &mut Self {
        self.0.children.push(child.0);
        self
    }
}
impl From<ChmTopicBuilder> for ChmContentsEntry {
    fn from(value: ChmTopicBuilder) -> Self {
        value.0
    }
}