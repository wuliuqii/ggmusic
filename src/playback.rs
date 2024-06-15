use std::sync::{atomic::AtomicUsize, Arc};

use gpui::{Context, Global, Model, WindowContext};
use rodio::{OutputStream, OutputStreamHandle, Sink};

use crate::metadata::track::Track;

pub struct Playback {
    playlist: Playlist,
    player: Player,
}

impl Playback {
    pub fn new() -> Self {
        let playlist = Playlist::default();
        let player = Player::new();

        Self { player, playlist }
    }
}

#[derive(Clone)]
pub struct PlaybackModel(Model<Playback>);

impl PlaybackModel {
    pub fn init(cx: &mut WindowContext) -> Self {
        let playback = Playback::new();
        let this = Self(cx.new_model(|_| playback));
        cx.set_global(this.clone());
        this
    }
}

impl Global for PlaybackModel {}

#[derive(Default)]
struct Playlist {
    tracks: Vec<Arc<Track>>,
    current: Option<usize>,
    playing: bool,
}

struct Player {
    sink: Arc<Sink>,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    playlist_length: Arc<AtomicUsize>,
}

impl Player {
    fn new() -> Self {
        let (_stream, _stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&_stream_handle).unwrap();
        let sink = Arc::new(sink);

        Self {
            sink,
            _stream,
            _stream_handle,
            playlist_length: Arc::new(AtomicUsize::new(0)),
        }
    }
}
