use audiotags::Tag;
use color_eyre::eyre::{eyre, Result};
use metadata::media_file::MediaFileMetadata;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Song {
    title: String,
    artist: String,
    album: String,
    track_number: u16,
    duration: u64,
    path: PathBuf,
}

#[allow(dead_code)]
#[derive(Default)]
pub struct SongBuilder {
    title: String,
    artist: String,
    album: String,
    track_number: u16,
    duration: u64,
    path: PathBuf,
}

#[allow(dead_code)]
impl Song {
    pub fn new() -> SongBuilder {
        SongBuilder::default()
    }
}

impl Song {
    #![allow(dead_code)]
    pub fn duration_in_minutes_and_seconds(&self) -> String {
        format!("{}m {}s", self.duration / 60, self.duration % 60)
    }

    pub fn print(&self) {
        println!(
            "Title: {}\nArtist: {}\nAlbum: {}\nLength: {}",
            self.title,
            self.artist,
            self.album,
            self.duration_in_minutes_and_seconds()
        );
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn track_number(&self) -> &u16 {
        &self.track_number
    }

    pub fn tags(&self) -> Vec<String> {
        let v = vec![
            self.title.clone(),
            self.artist.clone(),
            self.album.clone(),
            String::from(self.track_number.clone().to_string()),
        ];
        v
    }
}

#[allow(dead_code)]
impl SongBuilder {
    pub fn new() -> Self {
        SongBuilder {
            title: String::from(""),
            artist: String::from(""),
            album: String::from(""),
            track_number: 0,
            duration: 0,
            path: PathBuf::new(),
        }
    }

    pub fn from_path(mut self, path: &PathBuf) -> Result<SongBuilder> {
        if !path.exists() {
            return Err(eyre!("Song: Path does not exist"));
        }

        self.path = path.clone();

        let tag = Tag::new().read_from_path(path).unwrap();

        self.title = match tag.title() {
            Some(title) => title.to_string(),
            None => return Err(eyre!("Song: Title")),
        };

        let album = match tag.album() {
            Some(album) => album,
            None => return Err(eyre!("Song: album")),
        };

        self.album = album.title.to_string();

        let duration = match MediaFileMetadata::new(&self.path) {
            Ok(media) => media,
            Err(_e) => return Err(eyre!("Song: Duration")),
        };

        match album.artist {
            Some(artist) => self.artist = artist.to_owned(),
            None => panic!("return Err: Artist"),
        }

        match tag.track_number() {
            Some(t) => self.track_number = t,
            None => return Err(eyre!("Song: track number")),
        }

        match duration._duration {
            Some(duration) => self.duration = duration.ceil() as u64,
            None => return Err(eyre!("Song: duration")),
        }

        Ok(self)
    }

    pub fn build(self) -> Song {
        Song {
            title: self.title,
            artist: self.artist,
            album: self.album,
            track_number: self.track_number,
            duration: self.duration,
            path: self.path,
        }
    }
}
