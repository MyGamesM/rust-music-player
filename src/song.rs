use audiotags::{Album, Tag};
use color_eyre::eyre::{eyre, Result};
use metadata::media_file::MediaFileMetadata;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Song {
    title: String,
    artist: Option<String>,
    album: Option<String>,
    track_number: Option<u16>,
    duration: Option<u32>,
    path: PathBuf,
}

#[allow(dead_code)]
#[derive(Default)]
pub struct SongBuilder {
    title: String,
    artist: Option<String>,
    album: Option<String>,
    track_number: Option<u16>,
    duration: Option<u32>,
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
    pub fn duration_in_minutes_and_seconds(&self) -> Result<String> {
        let duration = match self.duration {
            Some(d) => d,
            None => return Err(eyre!("Song does not have a duration")),
        };

        Ok(format!("{}m {}s", duration / 60, duration % 60))
    }

    // pub fn print(&self) {
    //     println!(
    //         "Title: {}\nArtist: {}\nAlbum: {}\nLength: {}",
    //         self.title,
    //         self.artist,
    //         self.album,
    //         self.duration_in_minutes_and_seconds()
    //     );
    // }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn track_number(&self) -> Result<&u16> {
        match &self.track_number {
            Some(num) => Ok(num),
            None => Err(eyre!("Song does not have a track number")),
        }
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn get_file_name(&self) -> String {
        format!("{}", self.get_path().display())
            .split("/")
            .last()
            .unwrap()
            .to_owned()
    }

    // pub fn tags(&self) -> Vec<String> {
    //     let v = vec![
    //         self.title.clone(),
    //         self.artist.clone(),
    //         self.album.clone(),
    //         String::from(self.track_number.clone().to_string()),
    //     ];
    //     v
    // }
}

#[allow(dead_code)]
impl SongBuilder {
    pub fn new() -> Self {
        SongBuilder {
            title: String::from(""),
            artist: Some(String::from("")),
            album: Some(String::from("")),
            track_number: Some(0),
            duration: Some(0),
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
            None => return Err(eyre!("Song: title")),
        };

        self.track_number = match tag.track_number() {
            Some(t) => Some(t),
            None => None,
        };

        let album: Option<Album<'_>> = match tag.album() {
            Some(album) => Some(album),
            None => None,
        };

        self.album = match &album {
            Some(album) => Some(album.title.to_string()),
            None => None,
        };

        match album {
            Some(album) => {
                match album.artist {
                    Some(artist) => self.artist = Some(artist.to_owned()),
                    None => return Err(eyre!("Song: artist")),
                };
            }
            None => {}
        }

        let duration: Option<Result<MediaFileMetadata, std::io::Error>> =
            match MediaFileMetadata::new(&self.path) {
                Ok(media) => Some(Ok(media)),
                Err(_) => None,
            };

        self.duration = match &duration {
            Some(duration) => match duration {
                Ok(duration) => match duration._duration {
                    Some(duration) => Some(duration.ceil() as u32),
                    None => None,
                },
                Err(_e) => None,
            },
            None => None,
        };

        match duration.unwrap().unwrap()._duration {
            Some(duration) => self.duration = Some(duration.ceil() as u32),
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
