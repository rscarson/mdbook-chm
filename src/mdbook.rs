use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};

use crate::{ChmBuilder, ChmLanguage, ChmTopicBuilder, MakeAbsolute, SafeWrite};
use mdbook::{book::Chapter, renderer::RenderContext};

pub fn get_context() -> Option<RenderContext> {
    RenderContext::from_json(&mut std::io::stdin()).ok()
}

pub fn context_to_chm(ctx: RenderContext) -> ChmBuilder {
    let cfg: Config = ctx
        .config
        .get_deserialized_opt("output.chm")
        .ok()
        .flatten()
        .unwrap_or_default();

    let title = ctx
        .config
        .book
        .title
        .clone()
        .unwrap_or_else(|| "Book".to_string());
    let lang = ChmLanguage::from_code(&cfg.language_code.to_lowercase()).unwrap_or_default();
    let output_path = ctx.root.make_absolute().join(&cfg.output_path);

    let mut builder = ChmBuilder::new(title, lang, output_path);
    for item in ctx.book.iter() {
        if let mdbook::BookItem::Chapter(chapter) = item {
            if let Some(topic) = chapter_to_chm(&ctx, chapter, 0) {
                builder = builder.with_contents(topic);
            }
        }
    }

    builder
}

fn chapter_to_chm(ctx: &RenderContext, chapter: &Chapter, depth: usize) -> Option<ChmTopicBuilder> {
    let tabs = "  ".repeat(depth);
    let path = chapter.path.as_ref()?;
    let title = chapter.name.clone();
    let html_path = path.with_extension("html");
    let html_root_path = ctx.root.make_absolute().join("book").join("html");
    let chm_root_path = ctx.root.make_absolute().join("book").join("chm");

    println!("{tabs}Processing `{}`", path.display());

    //
    // We need to export the modified pages to the output directory
    let page = Page::from_html(&html_path, &html_root_path).ok()?;
    let output_path = chm_root_path.join("html").join(&page.filename);
    let page_content = page.render();
    output_path.safe_write(&page_content).ok()?;
    println!("{tabs}  Render successful");

    //
    // Now we copy any referenced images to the output directory
    for image_path in &page.image_paths {
        let in_path = html_root_path.join(image_path);

        //
        // Now we need to remove the root/book/html/ part of the path
        let image_path = in_path
            .strip_prefix(&html_root_path)
            .ok()
            .unwrap_or(&in_path);

        //
        // And copy it to the output directory
        let output_image_path = chm_root_path.join("html").join(image_path);
        output_image_path.safe_copy(&in_path).ok()?;
    }
    println!("  Copied {} images", page.image_paths.len());

    let mut topic = ChmTopicBuilder::new(title, output_path.to_string_lossy());
    for subchapter in &chapter.sub_items {
        if let mdbook::BookItem::Chapter(subchapter) = subchapter {
            let subtopic = match chapter_to_chm(ctx, subchapter, depth + 1) {
                Some(subtopic) => subtopic,
                None => {
                    println!("Error in subchapter `{:?}`", subchapter.path);
                    return None;
                }
            };
            topic.with_child(subtopic);
        }
    }

    Some(topic)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
    language_code: String,
    output_path: String,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            language_code: "en-us".to_string(),
            output_path: "book/chm/output.chm".to_string(),
        }
    }
}

pub struct Page {
    filename: PathBuf,
    title: String,
    contents: String,
    stylesheets: Vec<String>,
    image_paths: Vec<PathBuf>,
}
impl Page {
    pub fn from_html(path: &Path, root_dir: &Path) -> std::io::Result<Self> {
        let contents = match std::fs::read_to_string(root_dir.join(path)) {
            Ok(contents) => contents,
            Err(e) => {
                println!("Error reading `{}`: {e}", path.display());
                return Err(e);
            }
        };

        //
        // Get the title from the <title> tag.
        let title_reg = regex::Regex::new(r#"<title>(.*?)</title>"#).unwrap();
        let title = title_reg
            .captures(&contents)
            .and_then(|cap| cap.get(1))
            .map(|c| c.as_str())
            .unwrap_or("")
            .to_string();

        //
        // We also need any paths under <link rel="stylesheet" href="..."> tags.
        let link_reg = regex::Regex::new(r#"<link rel="stylesheet" href="(.*?)">"#).unwrap();
        let stylesheets = link_reg
            .captures_iter(&contents)
            .filter_map(|cap| cap.get(1))
            .map(|c| c.as_str().to_string())
            .collect();

        //
        // The body will contain a <main> tag. The inner contents of the <main> tag will be the contents of the page.
        let main_reg = regex::Regex::new(r#"(?s)<main>(.*?)</main>"#).unwrap();
        let body = main_reg
            .captures(&contents)
            .and_then(|cap| cap.get(1))
            .map(|c| c.as_str());

        //
        // Make sure we have a body
        let contents = match body {
            Some(contents) => contents.to_string(),
            None => {
                println!("No <main> tag found in HTML file: {}", path.display());
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "No <main> tag found in HTML file",
                ));
            }
        };

        //
        // Any images will be in <img src="..."> tags, with paths relative to the path parent
        let root = path.parent();
        let img_reg = regex::Regex::new(r#"<img[^>]*\s+src=["']([^"']+)["'][^>]*>"#).unwrap();
        let image_paths = img_reg
            .captures_iter(&contents)
            .filter_map(|cap| cap.get(1))
            .map(|c| c.as_str())
            .map(|s| {
                let path = Path::new(s);
                if path.is_absolute() {
                    path.to_path_buf()
                } else if let Some(root) = root {
                    root.join(path)
                } else {
                    path.to_path_buf()
                }
            })
            .collect::<Vec<_>>();

        let filename: PathBuf = path.to_path_buf();
        Ok(Self {
            filename,
            title,
            contents,
            stylesheets,
            image_paths,
        })
    }

    pub fn render(&self) -> String {
        let mut cache = get_style_cache().lock().unwrap();
        let cache = cache.get_mut();
        let stylesheets = self.stylesheets.iter().map(|s| {
            //
            // First search the cache
            if let Some((_, contents)) = cache.iter().find(|(p, _)| p == s) {
                return format!("<style>{contents}</style>");
            }

            //
            // If not found, read the file and add it to the cache
            let contents = std::fs::read_to_string(s).unwrap_or_default();
            cache.push((s.clone(), contents.clone()));
            format!("<style>{contents}</style>")
        });

        let mut buffer = String::new();
        buffer.push_str("<!DOCTYPE html><html><head>");
        for stylesheet in stylesheets {
            buffer.push_str(&stylesheet);
        }
        buffer.push_str("<meta name=\"GENERATOR\" content=\"@rscarson&reg; mdbook-chm\">");
        buffer.push_str("<meta charset=\"utf-8\">");
        buffer.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
        buffer.push_str("<title>");
        buffer.push_str(&self.title);
        buffer.push_str("</title>");
        buffer.push_str("</head><body>");
        buffer.push_str(&self.contents);
        buffer.push_str("</body></html>");

        buffer
    }
}

//
// Static stylesheet cache
type StyleCache = Vec<(String, String)>;
static STYLE_CACHE: OnceLock<Mutex<RefCell<StyleCache>>> = OnceLock::new();
fn get_style_cache() -> &'static Mutex<RefCell<StyleCache>> {
    STYLE_CACHE.get_or_init(|| Mutex::new(RefCell::new(vec![])))
}
