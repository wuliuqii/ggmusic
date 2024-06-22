use std::sync::Arc;

use gpui::{
    div, px, FontWeight, Model, ParentElement, Render, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

use crate::{
    browse::Browse, events::UiEvent, metadata::library::LibraryModel, playback::Playback,
    playing::Playing, theme::Theme,
};

pub struct Root {
    browse: View<Browse>,
    playback: Model<Playback>,
    playing: View<Playing>,
    // memu: View<Menu>,
}

impl Root {
    pub fn new(cx: &mut WindowContext) -> Self {
        let library = LibraryModel::init(cx);

        let playback = Playback::init(cx);

        let browse = Browse::init(cx, library.clone());
        let playing = Playing::init(cx, library.clone());

        Self {
            browse,
            playback,
            playing,
        }
    }

    fn handle_ui_event(&mut self, event: &Arc<UiEvent>, cx: &mut WindowContext) {
        match (**event).clone() {
            UiEvent::PlayClicked(event) => self.playback.update(cx, |this, cx| {
                this.play(Arc::clone(&event.track), cx);
                cx.notify();
            }),
            UiEvent::PauseClicked => self.playback.update(cx, |this, cx| {
                this.pause(cx);
                cx.notify();
            }),
        };
    }
}

impl Render for Root {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .size_full()
            .flex()
            .flex_col()
            .min_h_0()
            .p_1()
            .bg({
                let mut bg = theme.base;
                bg.fade_out(0.1);
                bg
            })
            .child(div().min_h(px(30.)))
            .child(
                div()
                    .flex_grow()
                    .flex()
                    .min_h_0()
                    .gap(px(2.))
                    .rounded_md()
                    .p(px(2.))
                    .font_family(theme.font_mono.clone())
                    .font_weight(FontWeight::MEDIUM)
                    .child(self.browse.clone())
                    .child(self.playing.clone()),
            )
    }
}
