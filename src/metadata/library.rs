use std::{path::PathBuf, sync::Arc};

use gpui::{Context, Global, Model, WindowContext};

use super::track::{Track, Tracks};

const LIBRARY_PATH: &str = "/home/gallon/Music";

#[derive(Debug, Clone)]
pub struct Library(Vec<Arc<Track>>);

impl Library {
    pub fn load_tracks() -> Self {
        let dir = PathBuf::from(LIBRARY_PATH.to_string());
        let mut tracks = vec![];
        for entry in dir.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let track = Track::read(&path).unwrap();
                if track.is_song() {
                    tracks.push(Arc::new(track));
                }
            }
        }

        Library(tracks)
    }
}

#[derive(Clone)]
pub struct LibraryModel(Model<Library>);

impl LibraryModel {
    pub fn init(cx: &mut WindowContext) -> Self {
        let library = Library::load_tracks();
        let this = Self(cx.new_model(|_| library));
        cx.set_global(this.clone());
        this
    }

    pub fn get_tracks(&self, cx: &mut WindowContext) -> Tracks {
        let tracks = self.0.read(cx).0.clone();
        Tracks { tracks }
    }
}

impl Global for LibraryModel {}
