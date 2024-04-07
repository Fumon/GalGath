use std::{fmt::Display, sync::Arc};

#[derive(Default)]
pub(crate) struct EbookState(pub Arc<tokio::sync::RwLock<Option<RenderedEpub>>>);

impl Clone for EbookState {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

#[derive(Default, Clone)]
pub(crate) struct RenderedEpub {
    pub title: String,
    pub author: String,
    pub pages: i32,
    pub ebook: Vec<u8>,
}

impl Display for RenderedEpub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}; Author: {}; Page Count: {}",
            self.title, self.author, self.pages
        )
    }
}