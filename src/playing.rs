use std::sync::Arc;

use gpui::{
    div, img, InteractiveElement, ParentElement, Render, Styled, View, VisualContext, WindowContext,
};

use crate::metadata::{
    library::LibraryModel,
    track::{self, Track},
};

pub struct Playing {
    current_track: Option<Track>,
}

impl Playing {
    pub fn init(cx: &mut WindowContext, model: LibraryModel) -> View<Self> {
        let current_track = model.get_tracks(cx).tracks.first().cloned();
        cx.new_view(|_| Self { current_track })
    }
}

impl Render for Playing {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        let e = div().flex().flex_grow().flex_col().max_w_80().child(
            div()
                .id("track-info")
                .flex_grow()
                .flex()
                .flex_col()
                .gap_px()
                .rounded_b_sm()
                .child(
                    div().py_1().px_3().children([
                        // TODO: avoid clone
                        self.current_track
                            .clone()
                            .map_or("-".to_string(), |track| track.title),
                        self.current_track
                            .clone()
                            .map_or("-".to_string(), |track| track.artist),
                        self.current_track
                            .clone()
                            .map_or("-".to_string(), |track| track.album),
                    ]),
                ),
        );

        let e = if let Some(track) = self.current_track.clone() {
            if let Some(cover) = track.cover.clone() {
                e.child(img(cover).flex_none().w_80().h_80())
            } else {
                e
            }
        } else {
            e
        };

        e.child(
            div().flex().mt_auto().gap_px().children([
                div()
                    .id("pause")
                    .flex_1()
                    .py_1()
                    .px_3()
                    .flex()
                    .justify_center()
                    .child("Pause"),
                div()
                    .id("next")
                    .flex_1()
                    .py_1()
                    .px_3()
                    .flex()
                    .justify_center()
                    .child("Next"),
            ]),
        )
    }
}
