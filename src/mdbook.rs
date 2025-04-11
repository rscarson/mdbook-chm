use crate::chm::{ChmBuilder, ChmTopicBuilder, language::ChmLanguage, utilities::MakeAbsolute};
use mdbook::{BookItem, renderer::RenderContext};
use std::{collections::HashSet, path::Path};

/// Get the current context from the command line arguments.
pub fn context() -> Option<RenderContext> {
    let ctx = RenderContext::from_json(&mut std::io::stdin()).ok();

    //
    // Set working directory to the book root
    if let Some(ctx) = &ctx {
        let book_root = ctx.root.clone();
        std::env::set_current_dir(&book_root).ok()?;
    }

    ctx
}

/// Trait to convert the current context to a CHM builder.
pub trait MdBookChm {
    fn chm_config(&self) -> MdbookChmConfig;

    fn topics(&self) -> std::io::Result<Vec<ChmTopicBuilder>>;
    fn as_chm(&self) -> std::io::Result<ChmBuilder>;
}
impl MdBookChm for RenderContext {
    fn chm_config(&self) -> MdbookChmConfig {
        self.config
            .get_deserialized_opt("output.chm")
            .ok()
            .flatten()
            .unwrap_or_default()
    }

    fn topics(&self) -> std::io::Result<Vec<ChmTopicBuilder>> {
        let mut visited_topics = HashSet::new();
        let mut topics = Vec::new();

        for item in self.book.iter() {
            if let Some(topic) = item.as_topic(&mut visited_topics) {
                topics.push(topic?);
            }
        }

        Ok(topics)
    }

    fn as_chm(&self) -> std::io::Result<ChmBuilder> {
        let config = self.chm_config();
        let title = self.config.book.title.as_deref().unwrap_or("Book");

        //
        // Get language for the output
        let lang_code = config.language_code.to_lowercase();
        let lang = ChmLanguage::from_code(&lang_code).unwrap_or_default();

        //
        // Get path definitions
        let root = self.root.make_absolute().join(&self.destination);
        let output_path = root.join(&config.output_path);

        let mut builder = ChmBuilder::new(title, lang, output_path);

        //
        // Add topics
        for topic in self.topics()? {
            builder = builder.with_contents(topic)?;
        }

        Ok(builder)
    }
}

trait AsTopic {
    fn as_topic<'a>(
        &'a self,
        visited_topics: &mut HashSet<&'a Path>,
    ) -> Option<std::io::Result<ChmTopicBuilder>>;
}
impl AsTopic for mdbook::BookItem {
    fn as_topic<'a>(
        &'a self,
        visited_topics: &mut HashSet<&'a Path>,
    ) -> Option<std::io::Result<ChmTopicBuilder>> {
        let BookItem::Chapter(chapter) = self else {
            return None;
        };

        let chapter_path = chapter.path.as_ref()?;

        if visited_topics.contains(chapter_path.as_path()) {
            return None;
        }

        visited_topics.insert(chapter_path);

        println!("Adding topic: {}", chapter_path.display());

        let mut topic = match ChmTopicBuilder::new(
            &chapter.name,
            Path::new("src").join(chapter_path.to_windows_path()),
        ) {
            Ok(topic) => topic,
            Err(e) => return Some(Err(e)),
        };
        for item in &chapter.sub_items {
            if let Some(subtopic) = item.as_topic(visited_topics) {
                match subtopic {
                    Ok(subtopic) => {
                        topic.with_child(subtopic);
                    }
                    Err(e) => {
                        return Some(Err(e));
                    }
                }
            }
        }

        Some(Ok(topic))
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct MdbookChmConfig {
    language_code: String,
    output_path: String,
}
impl Default for MdbookChmConfig {
    fn default() -> Self {
        Self {
            language_code: "en-us".to_string(),
            output_path: "book.chm".to_string(),
        }
    }
}
