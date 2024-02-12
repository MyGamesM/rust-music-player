use crate::song::Song;

#[derive(Debug)]
pub struct Queue {
    pub queue: Vec<Song>,
    pub playing: u32,
}

#[allow(dead_code)]
impl Queue {
    pub fn new() -> Queue {
        Queue {
            queue: Vec::new(),
            playing: 0,
        }
    }

    pub fn add(&mut self, song: Song) {
        self.queue.push(song);
    }

    pub fn next(&mut self) -> Option<Song> {
        if self.queue.is_empty() {
            None
        } else {
            return Some(self.queue.pop()?);
        }
    }

    pub fn next_clone(&self) -> Option<Song> {
        if self.queue.is_empty() {
            None
        } else {
            return Some(self.queue[0].clone());
        }
    }
}
