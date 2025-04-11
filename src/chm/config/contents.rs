use crate::chm::utilities::MakeAbsolute;
use comrak::{Arena, ComrakOptions, ExtensionOptions, nodes::NodeValue};
use std::{
    ffi::OsStr,
    io::BufWriter,
    path::{Path, PathBuf},
};

/// All the files included in the CHM file.
#[derive(Debug, Clone)]
pub struct IncludedFiles {
    pub files: Vec<File>,
}
impl Default for IncludedFiles {
    fn default() -> Self {
        Self::new()
    }
}
impl IncludedFiles {
    /// Creates a new `IncludedFiles` instance with the given source root and optional body regex.
    pub fn new() -> Self {
        Self { files: vec![] }
    }

    /// Adds a file to the list of included files, and processes it if it is an HTML file.
    pub fn add_file(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let src_path = path.as_ref();
        let mut dst_path = src_path.to_path_buf();

        println!("Processing `{}`", src_path.make_absolute().display());

        //
        // Make sure the path is valid
        if src_path.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Path is a directory: {}", src_path.display()),
            ));
        }

        let contents = match src_path.extension().and_then(OsStr::to_str) {
            Some("md") => {
                let mut options = ComrakOptions::default();
                let contents = std::fs::read_to_string(src_path)?;

                //
                // Enable a shitload of options
                options.extension = ExtensionOptions {
                    strikethrough: true,
                    tagfilter: false,
                    table: true,
                    autolink: true,
                    tasklist: true,
                    superscript: true,
                    footnotes: true,
                    description_lists: true,
                    multiline_block_quotes: true,
                    alerts: true,
                    math_dollars: true,
                    math_code: true,
                    wikilinks_title_after_pipe: true,
                    wikilinks_title_before_pipe: true,
                    underline: true,
                    subscript: true,
                    spoiler: true,
                    greentext: true,

                    ..Default::default()
                };

                //
                // Parse the contents
                let arena = Arena::new();
                let root = comrak::parse_document(&arena, &contents, &options);

                //
                // First get the html representation of the file
                let mut bw = BufWriter::new(Vec::new());
                comrak::format_html(root, &options, &mut bw)?;
                let html = String::from_utf8(bw.into_inner().unwrap()).unwrap();
                let html = HTML_TEMPLATE.replace("%BODY%", &html);

                //
                // We need to scan the tree and find all the images
                let mut images = vec![];
                for node in root.descendants() {
                    if let NodeValue::Image(ref image) = node.data.borrow().value {
                        let mut path = Path::new(&image.url).to_path_buf();

                        //
                        // Possibly relative, set to parent of the current file
                        if let Some(parent) = src_path.parent() {
                            path = parent.join(path);
                        }

                        images.push(path);
                    }
                }

                //
                // Return the html contents
                dst_path.set_extension("html");
                html.into_bytes()
            }

            _ => std::fs::read(path)?,
        };

        let path = dst_path;
        let file = File { path, contents };
        self.files.push(file);

        Ok(())
    }

    /// Adds a file to the list of included files, but does not process it.
    pub fn append(&mut self, other: Self) {
        self.files.extend(other.files);
    }
}

/// A file included in the CHM file.
#[derive(Debug, Clone)]
pub struct File {
    pub path: PathBuf,
    pub contents: Vec<u8>,
}
impl File {
    pub fn is_html(&self) -> bool {
        self.path.extension().and_then(OsStr::to_str) == Some("html")
    }
}

const HTML_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <meta name="GENERATOR" content="@rscarson® mdbook-chm">
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    
    <title>%TITLE%</title>
    
    <style>
        body {
            font-family: "Segoe UI", Tahoma, sans-serif;
            font-size: 16px;
            line-height: 1.6;
            margin: 2em;
            color: #222;
            background-color: #fff;
        }
        h1, h2, h3, h4, h5 {
            margin-top: 1.5em;
            margin-bottom: 0.5em;
            font-weight: 600;
        }
        code, pre {
            background: #f4f4f4;
            font-family: Consolas, monospace;
            padding: 2px 4px;
            border-radius: 4px;
        }
        pre {
            padding: 1em;
            overflow-x: auto;
        }
        a {
            color: #0645ad;
            text-decoration: none;
        }
        a:hover {
            text-decoration: underline;
        }
        ul, ol {
            padding-left: 2em;
        }
        blockquote {
            margin: 1em 0;
            padding-left: 1em;
            border-left: 4px solid #ccc;
            color: #666;
        }
        table {
            border-collapse: collapse;
            width: 100%;
            margin-top: 1em;
        }
        th, td {
            border: 1px solid #ddd;
            padding: 0.5em;
            text-align: left;
        }
        th {
            background-color: #f9f9f9;
        }
    </style>
</head>
<body>
    %BODY%
</body>
</html>
"#;
