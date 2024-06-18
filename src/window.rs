use std::time::Duration;

use gpui::{
    div, AsyncAppContext, AsyncWindowContext, BorrowAppContext, Context, Global, IntoElement,
    Render, View, ViewContext, VisualContext, WindowContext,
};

use crate::theme::Theme;

struct NoView;

impl Render for NoView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
    }
}

pub struct Window {
    inner: View<NoView>,
    hidden: bool,
}

impl Window {
    pub fn init(cx: &mut WindowContext) {
        let view = cx.new_view(|cx| {
            cx.observe_window_activation(|_, cx| {
                if cx.is_window_active() {
                    return;
                };
                Self::close(cx);
            })
            .detach();

            cx.observe_window_appearance(|_, cx| {
                cx.update_global::<Theme, _>(|theme: &mut Theme, cx| {
                    *theme = Theme::mode(cx.window_appearance());
                    cx.refresh();
                });
            })
            .detach();

            NoView {}
        });

        cx.set_global::<Self>(Self {
            inner: view,
            hidden: false,
        });
    }

    pub fn is_open(cx: &AsyncAppContext) -> bool {
        cx.read_global::<Self, _>(|w, _| !w.hidden).unwrap_or(false)
    }

    pub fn open(cx: &mut WindowContext) {
        cx.update_global::<Self, _>(|this, cx| {
            if this.hidden {
                cx.activate_window();
                this.hidden = false;
            }
        });
    }

    pub fn close(cx: &mut WindowContext) {
        cx.update_global::<Self, _>(|this, cx| {
            this.hidden = true;
            cx.hide();
        });
        // After 90 seconds, reset the state
        cx.spawn(|mut cx| async move {
            cx.background_executor()
                .timer(Duration::from_secs(90))
                .await;
        })
        .detach();
    }

    pub async fn wait_for_close(cx: &mut AsyncWindowContext) {
        while let Ok(active) =
            cx.update_window::<bool, _>(cx.window_handle(), |_, cx| cx.is_window_active())
        {
            if !active {
                break;
            }
            cx.background_executor()
                .timer(Duration::from_millis(10))
                .await;
        }
    }
}

impl Global for Window {}
