mod app;
mod browse;
mod menu;
mod playback;
mod playing;
mod root;
mod theme;

mod metadata;

fn main() {
    env_logger::init();

    app::run_app();
}
