use std::{hash::Hash, sync::Arc};

use gpui::{div, ElementId, InteractiveElement, IntoElement, ParentElement, Render, Styled};
use lofty::picture::Picture;

use super::track::Track;

pub struct Album {
    title: String,
    artist: String,
    year: u32,
    tracks: Vec<Arc<Track>>,
    picture: Option<Picture>,
}

impl Hash for Album {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.title.hash(state);
        self.artist.hash(state);
    }
}

impl PartialEq for Album {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.artist == other.artist
    }
}

impl Eq for Album {}

impl Render for Album {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        div()
            .id(ElementId::Name(self.title.clone().into_element()))
            .size_64()
            .child(div().child("Cover"))
            .child(div().child(self.title.clone()))
            .child(div().child(self.artist.clone()))
    }
}

pub struct Albums {
    pub albums: Vec<Arc<Album>>,
}
