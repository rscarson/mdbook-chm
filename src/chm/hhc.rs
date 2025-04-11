//! The help table of contents (.hhc) file is an HTML file that contains the topic titles for your table of contents.
//! When a user opens the table of contents in a compiled help file (or on a Web page) and clicks a topic title, the HTML file associated with that title will open.

/// The TOC for the CHM file.
#[derive(Debug, Clone)]
pub struct ChmContents(pub Vec<ChmContentsEntry>);
impl ChmContents {
    pub const HEADER: &'static str = concat!(
        r#"<!DOCTYPE HTML PUBLIC "-//IETF//DTD HTML//EN">"#,
        r#"<HTML>"#,
        r#"<HEAD>"#,
        r#"<meta name="GENERATOR" content="@rscarson&reg; mdbook-chm">"#,
        r#"<!-- Sitemap 1.0 -->"#,
        r#"</HEAD><BODY>"#,
        r#"<OBJECT type="text/site properties">"#,
        r#"    <param name="Type" value=" ">"#,
        r#"    <param name="TypeDesc" value=" ">"#,
        r#"    <param name="Window Styles" value="0x800025">"#,
        r#"</OBJECT>"#,
    );
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
    pub title: String,
    pub file: String,

    pub children: Vec<ChmContentsEntry>,
}
impl ChmContentsEntry {
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
    pub fn flatten(mut self) -> Vec<Self> {
        let mut result = vec![];
        for child in self.children.drain(..) {
            result.extend(child.flatten());
        }

        result.push(self);
        result
    }
}
