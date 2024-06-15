use gpui::{App, WindowOptions};

use crate::root::Root;

pub fn run_app() {
    App::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |cx| Root::new(cx));
    });
}
