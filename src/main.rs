mod app;
mod asserts;
mod browse;
mod events;
mod menu;
mod playback;
mod playing;
mod root;
mod theme;
mod window;

mod metadata;

fn main() {
    env_logger::init();

    app::run_app();
}
