use gpui::{
    px, size, App, Bounds, VisualContext, WindowBackgroundAppearance, WindowBounds, WindowOptions,
};

use crate::{asserts::Assets, root::Root, theme::Theme, window::Window};

pub fn run_app() {
    App::new().with_assets(Assets).run(move |cx| {
        Theme::init(cx);

        let window_options = WindowOptions {
            titlebar: None,
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                size: size(px(800.), px(600.)).into(),
                ..Default::default()
            })),
            app_id: Some("ggmusic".to_string()),
            ..Default::default()
        };

        cx.open_window(window_options, |cx| {
            let theme = cx.global::<Theme>();
            cx.set_background_appearance(WindowBackgroundAppearance::from(
                theme.window_background.clone().unwrap_or_default(),
            ));

            let view = cx.new_view(|cx| Root::new(cx));
            Window::init(cx);
            view
        });
    });
}
