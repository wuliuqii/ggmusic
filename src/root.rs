use gpui::{
    div, font, px, rgb, ParentElement, Render, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

use crate::{
    browse::Browse, menu::Menu, metadata::library::LibraryModel, playback::PlaybackModel,
    playing::Playing, theme,
};

pub struct Root {
    playback: PlaybackModel,
    library: LibraryModel,
    browse: View<Browse>,
    // playing: View<Playing>,
    // memu: View<Menu>,
}

impl Root {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        let playback = PlaybackModel::init(cx);
        let library = LibraryModel::init(cx);

        let browse = Browse::init(cx, library.clone());

        cx.new_view(|cx| Self {
            playback,
            library,
            browse,
        })
    }
}

impl Render for Root {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .min_h_0()
            .p_1()
            .child(div().min_h(px(30.)))
            .child(
                div()
                    .flex_grow()
                    .flex()
                    .min_h_0()
                    .gap(px(2.))
                    .bg(rgb(theme::colours::STILL))
                    .rounded_md()
                    .p(px(2.))
                    .text_color(rgb(theme::colours::WINTER))
                    .font(font("JetBrains Mono"))
                    .child(self.browse.clone()),
            )
    }
}
