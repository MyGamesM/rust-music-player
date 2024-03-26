use crate::song::{Song, SongBuilder};
use color_eyre::eyre::{eyre, Result};
use permutation::permutation;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Playlist {
    name: String,
    length: u32,
    songs: Vec<Song>,
}

#[allow(dead_code)]
pub struct PlaylistBuilder {
    name: String,
    length: u32,
    songs: Vec<Song>,
}

#[allow(dead_code)]
impl Playlist {
    pub fn songs(&self) -> &Vec<Song> {
        &self.songs
    }
}

#[allow(dead_code)]
impl PlaylistBuilder {
    pub fn new() -> PlaylistBuilder {
        PlaylistBuilder {
            name: String::from(""),
            length: 0,
            songs: vec![],
        }
    }

    pub fn from_path(mut self, path: &str) -> Result<PlaylistBuilder> {
        if !PathBuf::from(path).exists() {
            return Err(eyre!("Path does not exist"));
        }

        // let names = match fs::read_dir(path) {
        //     Ok(path) => path,
        //     Err(e) => Err(eyre!("Playlist: {}", e)),
        // }

        let names = fs::read_dir(path)?
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    e.path()
                        .file_name()
                        .and_then(|n| n.to_str().map(|s| String::from(s)))
                })
            })
            .collect::<Vec<String>>();

        self.songs = names
            .iter()
            .map(|name| {
                SongBuilder::new()
                    .from_path(&PathBuf::from(format!("{}/{}", path, name)))
                    .expect("REASON")
                    .build()
            })
            .collect::<Vec<Song>>();

        self.name = match path.split("/").last() {
            Some(s) => s.to_string(),
            None => return Err(eyre!("Playlist: splitting path")),
        };

        self.length = self.songs.len() as u32;

        Ok(self)
    }

    pub fn sort_by_track_number(mut self) -> PlaylistBuilder {
        let track_numbers: Vec<&u16> = self
            .songs
            .iter()
            .filter_map(|song| match song.track_number() {
                Ok(num) => Some(num),
                Err(_e) => None,
            })
            .collect();

        let permutation = permutation::sort(&track_numbers);

        self.songs = permutation.apply_slice(&self.songs);

        self
    }

    pub fn build(self) -> Playlist {
        Playlist {
            name: self.name,
            length: self.length,
            songs: self.songs,
        }
    }
}
