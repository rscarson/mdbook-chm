//! The project file (.hhp) is a text file that contains the project settings for your compiled help file.
//! It specifies the files to be included in the help file, the title of the help file,
//! and other settings such as the default window size and the location of the table of contents and index files.
//!
use crate::ChmLanguage;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ChmProject {
    pub title: String,
    pub language: ChmLanguage,

    pub output_path: String,
    pub index_path: String,
    pub contents_path: String,

    pub files: Vec<String>,
}
impl std::fmt::Display for ChmProject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = &self.title;
        let language = &self.language;
        let output_path = &self.output_path;
        let index_path = &self.index_path;
        let contents_path = &self.contents_path;
        let files = &self.files.join("\n");

        // Remove the file name from the output path
        let default_file = if let Some(file) = self.files.first() {
            let path = Path::new(file);
            if let Some(parent) = Path::new(output_path).parent() {
                path.strip_prefix(parent).unwrap_or(path)
            } else {
                path
            }
            .to_string_lossy()
            .to_string()
        } else {
            String::new()
        };

        write!(
            f,
            concat!(
                "[OPTIONS]\n",
                "Binary TOC=Yes\n",
                "Compatibility=1.1 or later\n",
                "Compiled file={output_path}\n",
                "Contents file={contents_path}\n",
                "Default topic={default_file}\n",
                "Display compile progress=Yes\n",
                "Enhanced decompilation=Yes\n",
                "Full-text search=Yes\n",
                "Index file={index_path}\n",
                "Language={language}\n",
                "Title={title}\n",
                "\n",
                "\n",
                "[FILES]\n",
                "{files}\n",
                "\n",
                "[INFOTYPES]"
            ),
            output_path = output_path,
            contents_path = contents_path,
            index_path = index_path,
            default_file = default_file,
            language = language,
            title = title,
            files = files,
        )
    }
}
