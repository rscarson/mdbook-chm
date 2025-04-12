//! The index (.hhk) file is an HTML file that contains the index entries (keywords) for your index.
//! When a user opens the index in a compiled help file, or on a Web page, and clicks a keyword, the HTML file associated with the keyword will open.

/// The index for the CHM project
#[derive(Debug, Clone)]
pub struct ChmIndex(pub Vec<ChmIndexEntry>);
impl ChmIndex {
    const HEADER: &'static str = concat!(
        r#"<!DOCTYPE HTML PUBLIC "-//IETF//DTD HTML//EN">"#,
        r#"<HTML>"#,
        r#"<HEAD>"#,
        r#"<meta name="GENERATOR" content="@rscarson&reg; mdbook-chm">"#,
        r#"<!-- Sitemap 1.0 -->"#,
        r#"</HEAD><BODY>"#,
    );
}
impl std::fmt::Display for ChmIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            body = self
                .0
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

/// A single entry in the index
#[derive(Debug, Clone)]
pub struct ChmIndexEntry {
    /// Keyword to display
    pub keyword: String,

    /// Path for the keyword
    pub file: String,
}
impl std::fmt::Display for ChmIndexEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            concat!(
                r#"    <LI> <OBJECT type="text/sitemap">"#,
                r#"        <param name="Name" value="{keyword}">"#,
                r#"        <param name="Local" value="{file}">"#,
                r#"        </OBJECT>"#,
            ),
            keyword = self.keyword,
            file = self.file,
        )
    }
}
