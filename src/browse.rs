use gpui::{
    div, px, InteractiveElement, ParentElement, Render, StatefulInteractiveElement, Styled,
    View, VisualContext, WindowContext,
};

use crate::{
    metadata::{library::LibraryModel, track::Tracks},
};

pub struct Browse {
    tracks: View<Tracks>,
    // albums: View<Albums>,
}

impl Browse {
    pub fn init(cx: &mut WindowContext, model: LibraryModel) -> View<Self> {
        let tracks = cx.new_view(|cx| model.get_tracks(cx));
        // let albums = model.get_albums(cx);
        cx.new_view(|cx| Self { tracks })
    }
}

impl Render for Browse {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        div()
            .id("browse-view")
            .flex_grow()
            .overflow_scroll()
            .rounded_b_sm()
            .p(px(1.))
            .child(self.tracks.clone())
    }
}
