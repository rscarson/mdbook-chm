use crate::chm::contents::File;
use comrak::{Arena, ComrakOptions, ExtensionOptions, nodes::NodeValue};
use std::{
    io::BufWriter,
    path::{Path, PathBuf},
};

/// Loads a mardown file, rendering it as HTML
///
/// # Errors
/// Can return an error if the source cannot be read
pub fn load(path: &Path, contents: &[u8]) -> std::io::Result<(File, Vec<PathBuf>)> {
    // Convert the contents to a string
    let contents = String::from_utf8_lossy(contents).to_string();

    let mut options = ComrakOptions::default();

    //
    // Enable a shitload of options
    options.render.unsafe_ = true;
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
    // A little preprocessing here
    // Replace `>\n` with `>  \n`
    let contents = contents.replace(">\n", ">  \n");

    //
    // Parse the contents
    let arena = Arena::new();
    let root = comrak::parse_document(&arena, &contents, &options);

    //
    // We need to scan the tree and find all the images
    let mut dependencies = vec![];
    for node in root.descendants() {
        if let NodeValue::Image(ref image) = node.data.borrow().value {
            if image.url.starts_with("http") {
                continue;
            }

            let mut img_path = Path::new(&image.url).to_path_buf();

            //
            // Possibly relative, set to parent of the current file
            if img_path.is_relative() {
                if let Some(parent) = path.parent() {
                    img_path = parent.join(&img_path);
                }
            }

            dependencies.push(img_path);
        }

        //
        // We also need to set the extension of all links pointing to relative .md files to .html
        if let NodeValue::Link(ref mut link) = node.data.borrow_mut().value {
            if !link.url.starts_with("http") {
                let path = Path::new(&link.url);
                if path.is_relative() {
                    link.url = path.with_extension("html").to_string_lossy().to_string();
                }
            }
        }
    }

    //
    // Get the html representation of the file
    let mut bw = BufWriter::new(Vec::new());
    comrak::format_html(root, &options, &mut bw)?;
    let html = String::from_utf8(bw.into_inner().unwrap_or_default()).unwrap_or_default();
    let html = HTML_TEMPLATE.replace("%BODY%", &html);

    //
    // Return the html contents
    let own_file = File {
        path: path.with_extension("html"),
        contents: html.as_bytes().to_vec(),
    };
    Ok((own_file, dependencies))
}

const HTML_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <meta name="GENERATOR" content="@rscarson® mdbook-chm">
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta charset="utf-8">    
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
