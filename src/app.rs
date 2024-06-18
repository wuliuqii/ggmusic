use gpui::{App, WindowBackgroundAppearance, WindowOptions};

use crate::{asserts::Assets, root::Root, theme::Theme, window::Window};

pub fn run_app() {
    App::new().run(|cx| {
        App::new().with_assets(Assets).run(move |cx| {
            Theme::init(cx);

            cx.open_window(WindowOptions::default(), |cx| {
                let theme = cx.global::<Theme>();
                cx.set_background_appearance(WindowBackgroundAppearance::from(
                    theme.window_background.clone().unwrap_or_default(),
                ));

                let view = Root::build(cx);
                Window::init(cx);
                view
            });
        })
    });
}
