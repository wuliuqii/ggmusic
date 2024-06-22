use std::sync::Arc;

use crate::{
    browse::Browse,
    metadata::{
        album::Albums,
        track::{Track, Tracks},
    },
    playback::Playback,
};

#[derive(Clone)]
pub enum UiEvent {
    PlayClicked(PlayClickedEvent),
    PauseClicked,
}

impl UiEvent {
    pub fn play(track: &Arc<Track>) -> Arc<UiEvent> {
        Arc::new(UiEvent::PlayClicked(PlayClickedEvent {
            track: Arc::clone(track),
        }))
    }
}
impl gpui::EventEmitter<Arc<UiEvent>> for Albums {}
impl gpui::EventEmitter<Arc<UiEvent>> for Browse {}
impl gpui::EventEmitter<Arc<UiEvent>> for Tracks {}

#[derive(Clone)]
pub struct PlayClickedEvent {
    pub track: Arc<Track>,
}

#[derive(Clone)]
pub enum PlaybackEvent {
    TrackStarted(Arc<Track>),
    TrackEnded,
    Paused,
    Resumed,
}

impl PlaybackEvent {
    pub fn start(track: &Arc<Track>) -> Arc<PlaybackEvent> {
        Arc::new(PlaybackEvent::TrackStarted(Arc::clone(track)))
    }
}

impl gpui::EventEmitter<Arc<PlaybackEvent>> for Playback {}
