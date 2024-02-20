use crate::song::{Song, SongBuilder};
use anyhow::{bail, Result};
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
            bail!("Path does not exist")
        }

        let paths = fs::read_dir(path).unwrap();

        let names = paths
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
                match SongBuilder::new().from_path(&PathBuf::from(format!("{}/{}", path, name))) {
                    Ok(song) => song.build(),
                    Err(e) => panic!("{}", e),
                }
            })
            .collect::<Vec<Song>>();

        self.name = match path.split("/").last() {
            Some(s) => s.to_string(),
            None => bail!("Playlist: splitting path"),
        };

        self.length = self.songs.len() as u32;

        Ok(self)
    }

    pub fn build(self) -> Playlist {
        Playlist {
            name: self.name,
            length: self.length,
            songs: self.songs,
        }
    }
}
