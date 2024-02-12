mod queue;
mod song;

use queue::Queue;
use song::Song;
use std::path::PathBuf;

#[allow(dead_code)]
enum Screen {
    QUEUE,
    PLAYLISTS,
    BROWSER,
}

fn main() {
    let mut q: Queue = Queue {
        queue: vec![],
        playing: 0,
    };

    let path = PathBuf::from(
        "/mnt/hdd/Music/Albums/Bullet Hell II/RichaadEB - Emotional Skyscraper ~ Cosmic Mind.mp3",
    );

    let song = Song::new(path);

    q.add(song);

    q.next_clone().unwrap().print();
}
