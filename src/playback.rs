use std::sync::atomic::Ordering::SeqCst;
use std::time::Duration;
use std::{
    fs::File,
    io::BufReader,
    sync::{atomic::AtomicUsize, Arc},
};

use gpui::{AppContext, Context, Global, Model, ModelContext, WindowContext};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

use crate::{events::PlaybackEvent, metadata::track::Track};

const POLL_DURATION: Duration = Duration::from_millis(100);

pub struct Playback {
    queue: Queue,
    player: Player,
}

impl Playback {
    pub fn init(cx: &mut WindowContext) -> Model<Self> {
        let queue = Queue::default();
        let player = Player::new();

        

        cx.new_model(|_cx| Self { player, queue })
    }

    pub fn play(&mut self, track: Arc<Track>, cx: &mut ModelContext<Self>) {
        if self.queue.current.is_some() {
            cx.emit(Arc::new(PlaybackEvent::TrackEnded));
        }

        self.queue.play(&track);
        self.player.play(&track, cx);

        cx.emit(PlaybackEvent::start(&track));
    }

    pub fn pause(&mut self, cx: &mut ModelContext<Self>) {
        self.player.pause(cx);
        self.queue.playing = false;

        cx.emit(Arc::new(PlaybackEvent::Paused));
    }

    fn on_track_end(&mut self, cx: &mut ModelContext<Self>) {
        let next = self.queue.get_next();
        cx.notify();

        cx.emit(Arc::new(PlaybackEvent::TrackEnded));

        if let Some(next) = next {
            cx.emit(PlaybackEvent::start(&next));
        }
    }
}

#[derive(Default)]
struct Queue {
    tracks: Vec<Arc<Track>>,
    current: Option<usize>,
    playing: bool,
}

impl Queue {
    fn play(&mut self, track: &Arc<Track>) {
        self.tracks = vec![Arc::clone(track)];
        self.current = Some(0);
        self.playing = true;
    }

    fn get_current(&self) -> Option<Arc<Track>> {
        self.current
            .and_then(|index| self.tracks.get(index))
            .cloned()
    }

    fn next(&mut self) {
        self.current = self.current.and_then(|index| {
            if index + 1 < self.tracks.len() {
                Some(index + 1)
            } else {
                None
            }
        });

        self.playing = self.current.is_some();
    }

    fn get_next(&mut self) -> Option<Arc<Track>> {
        self.next();
        self.get_current()
    }
}

struct Player {
    sink: Arc<Sink>,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    queue_len: Arc<AtomicUsize>,
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
            queue_len: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn watch(&self, cx: &mut ModelContext<Playback>) {
        let queue_len = Arc::clone(&self.queue_len);

        cx.spawn(|this, mut cx| async move {
            let mut prev_len = queue_len.load(SeqCst);

            loop {
                let current_len = queue_len.load(SeqCst);

                if current_len < prev_len {
                    this.update(&mut cx, |playback, cx| {
                        playback.on_track_end(cx);
                    })
                    .ok();
                }
                prev_len = current_len;
                cx.background_executor().timer(POLL_DURATION).await;
            }
        })
        .detach();
    }

    fn get_source(track: &Arc<Track>) -> Decoder<BufReader<File>> {
        let path = track.file.clone().unwrap();
        let file = BufReader::new(File::open(path).unwrap());
        Decoder::new(file).unwrap()
    }

    fn play(&self, track: &Arc<Track>, cx: &mut AppContext) {
        let track = Arc::clone(track);
        let sink = Arc::clone(&self.sink);
        let queue_len = Arc::clone(&self.queue_len);

        queue_len.store(1, SeqCst);

        cx.background_executor()
            .spawn(async move {
                let source = rodio::source::Done::new(Self::get_source(&track), queue_len);
                sink.clear();
                sink.append(source);
                sink.play();
            })
            .detach();
    }

    fn pause(&self, cx: &mut AppContext) {
        let sink = Arc::clone(&self.sink);
        cx.background_executor()
            .spawn(async move {
                sink.pause();
            })
            .detach();
    }
}
