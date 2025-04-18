//! The help table of contents (.hhc) file is an HTML file that contains the topic titles for your table of contents.
//! When a user opens the table of contents in a compiled help file (or on a Web page) and clicks a topic title, the HTML file associated with that title will open.
use super::contents::{File, IncludedFiles};
use crate::chm::utilities::MakeAbsolute;
use std::path::Path;

/// The TOC for the CHM file.
#[derive(Debug, Clone)]
pub struct ChmContents(pub Vec<ChmContentsEntry>);
impl ChmContents {
    const HEADER: &'static str = concat!(
        r#"<!DOCTYPE HTML PUBLIC "-//IETF//DTD HTML//EN">\n"#,
        r#"<HTML>\n"#,
        r#"<HEAD>\n"#,
        r#"<meta name="GENERATOR" content="@rscarson&reg; mdbook-chm">\n"#,
        r#"<!-- Sitemap 1.0 -->\n"#,
        r#"</HEAD><BODY>\n"#,
        r#"<OBJECT type="text/site properties">\n"#,
        r#"    <param name="Type" value=" ">\n"#,
        r#"    <param name="TypeDesc" value=" ">\n"#,
        r#"    <param name="Window Styles" value="0x800025">\n"#,
        r#"</OBJECT>\n"#,
    );

    /// Flatten this object into a list of entries instead of a tree
    ///
    /// This is used to turn it into an index, or list dependencies for the tree
    #[must_use]
    pub fn flatten(mut self) -> Vec<ChmContentsEntry> {
        let mut result = vec![];
        for entry in self.0.drain(..) {
            result.extend(entry.flatten());
        }
        result
    }
}
impl std::fmt::Display for ChmContents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let children = self.0.iter().map(|e| e.format(1)).collect::<Vec<_>>();

        write!(
            f,
            concat!(
                "{header}\n",
                "<UL>\n",
                "{body}\n",
                "</UL>\n",
                "</BODY></HTML>"
            ),
            header = Self::HEADER,
            body = children.join("\n")
        )
    }
}

/// A directory tree structure for the table of contents.
#[derive(Debug, Clone)]
pub struct ChmContentsEntry {
    /// The title of the chapter
    pub title: String,

    /// The path to the chapter contents
    pub file: String,

    /// Child topics
    pub children: Vec<ChmContentsEntry>,

    /// All included files for this chapter (not the children)
    pub files: Vec<File>,
}
impl ChmContentsEntry {
    /// Create a new entry based on a source file, and process dependencies
    ///
    /// # Errors
    /// Can return an error on IO failures
    pub fn new(title: &impl ToString, source: impl AsRef<Path>) -> std::io::Result<Self> {
        let contents = std::fs::read_to_string(source.as_ref())?;
        Self::with_contents(title, source, &contents)
    }
    /// Create a new entry based on a source file and contents, and process dependencies
    ///
    /// # Errors
    /// Can return an error on IO failures
    ///
    /// # Panics
    /// dont worry 'bout it kay?
    pub fn with_contents(
        title: &impl ToString,
        source: impl AsRef<Path>,
        contents: &str,
    ) -> std::io::Result<Self> {
        let mut files = IncludedFiles::new();
        files.add_file(source, contents.as_bytes())?;

        let own_path = &files.files.last().expect("We literally just added it").path;
        Ok(Self {
            title: title.to_string(),
            file: own_path.to_windows_path(),

            children: vec![],
            files: files.files.into_iter().collect(),
        })
    }

    /// Format the entry as a string.
    pub(crate) fn format(&self, depth: usize) -> String {
        let tabs = "\t".repeat(depth);
        let mut result = vec![
            format!("{tabs}<LI><OBJECT type=\"text/sitemap\">"),
            format!("{tabs}\t<param name=\"Name\" value=\"{}\">", self.title),
            format!("{tabs}\t<param name=\"Local\" value=\"{}\">", self.file),
            format!("{tabs}</OBJECT>"),
        ];

        if !self.children.is_empty() {
            result.push(format!("{tabs}<UL>"));
            for child in &self.children {
                result.push(child.format(depth + 1));
            }
            result.push(format!("{tabs}</UL>"));
        }

        result.join("\n")
    }

    /// Flattens the tree structure into a vector of entries.
    ///
    /// This is used to turn it into an index, or list dependencies for the tree
    #[must_use]
    pub fn flatten(mut self) -> Vec<Self> {
        let mut result = vec![];
        for child in self.children.drain(..) {
            result.extend(child.flatten());
        }

        result.push(self);
        result
    }
}
