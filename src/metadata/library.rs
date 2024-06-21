use rayon::prelude::*;
use std::{path::PathBuf, sync::Arc};

use gpui::{Context, Global, Model, WindowContext};

use super::track::{Track, Tracks};

const LIBRARY_PATH: &str = "/home/gallon/Music";

#[derive(Debug, Clone)]
pub struct Library(Vec<Track>);

impl Library {
    pub fn load_tracks() -> Self {
        let dir = PathBuf::from(LIBRARY_PATH.to_string());
        let mut entries = vec![];
        for entry in dir.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            entries.push(path);
        }

        // TODO: more elegant way to do this?
        let tracks = entries
            .par_iter()
            .map(|entry| {
                if entry.is_file() {
                    let track = Track::read(entry).unwrap();
                    if track.is_song() {
                        Some(track)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .filter(|track| track.is_some())
            .map(|track| track.unwrap())
            .collect::<Vec<Track>>();

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
