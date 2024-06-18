use std::{borrow::Cow, ffi::OsStr, path::Path, sync::Arc, time::Duration};

use anyhow::Result;
use gpui::{
    div, px, rgb, Element, ElementId, InteractiveElement, ParentElement, Render, Styled, View,
    VisualContext, WindowContext,
};
use lofty::{
    file::{AudioFile, FileType, TaggedFileExt},
    picture::{Picture, PictureType},
    probe::Probe,
    tag::{Accessor, ItemKey},
};

use crate::theme::{self, Theme};

use super::library::LibraryModel;

#[derive(Debug, Clone)]
pub struct Track {
    artist: String,
    title: String,
    album: String,
    ext: Option<String>,
    file: Option<String>,
    duration: Duration,
    picture: Option<Picture>,
    cover: Option<String>,
    file_type: Option<FileType>,
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.file == other.file
    }
}

impl Track {
    pub fn is_song(&self) -> bool {
        self.file_type.is_some()
    }

    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        let probe = Probe::open(path)?;

        let mut song = Self::new(path);
        if let Ok(mut tagged_file) = probe.read() {
            let properties = tagged_file.properties();
            song.duration = properties.duration();
            song.file_type = Some(tagged_file.file_type());

            if let Some(tag) = tagged_file.primary_tag_mut() {
                if let Some(len_tag) = tag.get_string(&ItemKey::Length) {
                    song.duration = Duration::from_millis(len_tag.parse::<u64>()?);
                }

                song.artist = tag
                    .artist()
                    .map(Cow::into_owned)
                    .unwrap_or("UNKNOWN".to_string());
                song.album = tag
                    .album()
                    .map(Cow::into_owned)
                    .unwrap_or("UNKNOWN".to_string());
                song.title = tag
                    .title()
                    .map(Cow::into_owned)
                    .unwrap_or("UNKNOWN".to_string());

                let mut picture = tag
                    .pictures()
                    .iter()
                    .find(|pic| pic.pic_type() == PictureType::CoverFront)
                    .cloned();
                if picture.is_none() {
                    picture = tag.pictures().first().cloned();
                }
                song.picture = picture;
            }
        }

        Ok(song)
    }
}

impl Track {
    fn new<P: AsRef<Path>>(path: P) -> Self {
        let p = path.as_ref();
        let title = p
            .file_stem()
            .and_then(OsStr::to_str)
            .map(String::from)
            .unwrap_or("UNKNOWN".to_string());
        let ext = p.extension().and_then(OsStr::to_str).map(String::from);
        let file = Some(p.to_string_lossy().to_string());
        let duration = Duration::from_secs(0);
        Self {
            artist: String::new(),
            title,
            album: String::new(),
            ext,
            file,
            duration,
            cover: None,
            file_type: None,
            picture: None,
        }
    }
}

impl Track {
    fn render(&self, cx: &mut gpui::WindowContext, index: usize) -> impl gpui::IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id(ElementId::Name(self.title.clone().into()))
            .flex()
            .gap_3()
            .h_8()
            .items_center()
            .rounded(px(1.))
            .hover(|style| {
                let mut bg_hover = theme.mantle;
                bg_hover.fade_out(0.5);
                style.bg(bg_hover)
            })
            .child(
                div()
                    .flex()
                    .w_8()
                    .justify_end()
                    .text_color(theme.subtext0)
                    .child((index + 1).to_string()),
            )
            .child(div().child(self.title.clone()))
            .child(div().child(self.artist.clone()))
    }
}

pub struct Tracks {
    pub tracks: Vec<Arc<Track>>,
}

impl Tracks {
    pub fn new(cx: &mut WindowContext, model: &LibraryModel) -> View<Self> {
        cx.new_view(|cx| model.get_tracks(cx))
    }
}

impl Render for Tracks {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        div().flex().flex_col().gap(px(1.)).children(
            self.tracks
                .iter()
                .enumerate()
                .map(|(index, track)| track.render(cx, index)),
        )
    }
}
