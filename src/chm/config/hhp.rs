//! The project file (.hhp) is a text file that contains the project settings for your compiled help file.
//! It specifies the files to be included in the help file, the title of the help file,
//! and other settings such as the default window size and the location of the table of contents and index files.
//!
use super::language::ChmLanguage;

/// The CHM project file, which ties the whole room together
#[derive(Debug, Clone)]
pub struct ChmProject {
    /// Book title
    pub title: String,

    /// Language code to display
    pub language: ChmLanguage,

    /// Path for the resulting chm
    pub output_path: String,

    /// Path to the index file
    pub index_path: String,

    /// Path to the contents file
    pub contents_path: String,

    /// Default file when opening
    pub default_file: String,
}
impl std::fmt::Display for ChmProject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = &self.title;
        let language = &self.language;
        let output_path = &self.output_path;
        let index_path = &self.index_path;
        let contents_path = &self.contents_path;
        let default_file = &self.default_file;

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
                "\n",
                "[INFOTYPES]"
            ),
            output_path = output_path,
            contents_path = contents_path,
            index_path = index_path,
            language = language,
            title = title,
            default_file = default_file,
        )
    }
}
