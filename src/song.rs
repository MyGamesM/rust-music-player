use audiotags::Tag;
use metadata;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Song {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    duration: Option<u64>,
    path: Option<PathBuf>,
}

#[allow(dead_code)]
impl Song {
    pub fn new(path: PathBuf) -> Song {
        let mut song = Song {
            title: None,
            artist: None,
            album: None,
            duration: None,
            path: Some(path),
        };

        song.validate_path();
        song.load_tags();

        return song;
    }

    fn load_tags(&mut self) {
        let tag = Tag::new()
            .read_from_path(self.path.as_ref().unwrap())
            .unwrap();

        self.title = Some(tag.title().unwrap().to_owned());

        self.album = Some(tag.album().unwrap().title.to_string());

        match tag.album().unwrap().artist {
            Some(artist) => self.artist = Some(artist.to_owned()),
            None => panic!("Err: Artist"),
        }

        let duration = metadata::media_file::MediaFileMetadata::new(self.path.as_ref().unwrap())
            .unwrap()
            ._duration;

        match duration {
            Some(duration) => {
                self.duration = Some(duration.ceil() as u64);
            }
            None => panic!("Err: Duration"),
        }
    }

    fn validate_path(&self) -> bool {
        match &self.path.as_ref().unwrap().try_exists() {
            Ok(result) => {
                if *result {
                    return *result;
                } else {
                    panic!("Err: validate_path File does not exist!");
                }
            }
            Err(e) => panic!("Err: validate_path {}", e),
        }
    }

    pub fn duration_in_minutes_and_seconds(&self) -> String {
        return format!(
            "{}m {}s",
            self.duration.unwrap() / 60,
            self.duration.unwrap() % 60
        );
    }

    pub fn duration_in_seconds(&self) -> u64 {
        return self.duration.unwrap();
    }

    pub fn print(&self) {
        println!(
            "Title: {}\nArtist: {}\nAlbum: {}\nLength: {}",
            self.title.as_ref().unwrap(),
            self.artist.as_ref().unwrap(),
            self.album.as_ref().unwrap(),
            self.duration_in_minutes_and_seconds()
        );
    }

    pub fn title(&self) -> Option<String> {
        return self.title.clone();
    }

    pub fn tags(&self) -> Vec<Option<String>> {
        let v = vec![self.title.clone(), self.artist.clone(), self.album.clone()];
        return v;
    }
}
