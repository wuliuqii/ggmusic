use gpui::{
    div, font, px, rgb, FontWeight, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::{
    browse::Browse,
    menu::Menu,
    metadata::library::LibraryModel,
    playback::PlaybackModel,
    playing::Playing,
    theme::{self, Theme},
};

pub struct Root {
    playback: PlaybackModel,
    library: LibraryModel,
    browse: View<Browse>,
    // playing: View<Playing>,
    // memu: View<Menu>,
}

impl Root {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
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
                    // .text_color(theme.text)
                    .child(self.browse.clone()),
            )
    }
}
