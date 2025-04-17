//! Finds `rare` keywords in a document set.
//! Defined as any word appearing in only one document in the set
//!

use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

/// Finds `rare` keywords in a document set.
///
/// Defined as any word appearing in only one document in the set
#[derive(Debug, Clone, Default)]
pub struct Keyworder<'src> {
    keywords: Vec<KeywordProperties<'src>>,
    index: HashMap<&'src str, usize>,
}

impl<'src> Keyworder<'src> {
    /// Create a new keyworder
    #[must_use]
    pub fn new() -> Self {
        Self {
            keywords: Vec::new(),
            index: HashMap::new(),
        }
    }

    /// Process a document and add its keywords to the index
    pub fn process(&mut self, path: &'src Path, content: &'src str) {
        let word_regex = regex::Regex::new(r"\b[\w-]+\b").unwrap();
        let word_iter = word_regex.find_iter(content).map(|m| m.as_str());

        let mut seen = HashSet::new();
        for word in word_iter {
            if seen.contains(word) {
                continue;
            }
            seen.insert(word);

            if let Some(index) = self.index.get(word) {
                self.keywords[*index].seen_in.push(path);
            } else {
                self.keywords.push(KeywordProperties {
                    keyword: word,
                    seen_in: vec![path],
                });
                self.index.insert(word, self.keywords.len() - 1);
            }
        }
    }

    /// Get the keywords in the keyworder visible within the threshold
    #[must_use]
    pub fn visible_keywords(&self) -> Vec<&KeywordProperties<'src>> {
        let iter = self.keywords.iter();
        let mut visible: Vec<_> = iter.filter(|k| k.seen_in.len() == 1).collect();
        visible.sort_by(|a, b| a.keyword.cmp(b.keyword));
        visible
    }
}

/// An extracted keyword and its properties
#[derive(Debug, Clone)]
pub struct KeywordProperties<'src> {
    /// The keyword itself
    pub keyword: &'src str,

    /// The documents it appears in
    pub seen_in: Vec<&'src Path>,
}
