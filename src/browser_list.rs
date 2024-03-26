use crate::song::{Song, SongBuilder};
use crate::App;
use color_eyre::eyre::Result;
use memoize::memoize;
use permutation::permutation;
use ratatui::{prelude::*, widgets::*};
use std::fs::metadata;
use std::path::PathBuf;

#[derive(Clone, PartialEq)]
pub enum FileType {
    FILE,
    DIRECTORY,
    NONE,
}

pub struct BrowserState {
    path: PathBuf,
    items: Vec<String>,
    state: ListState,
    file_type: Option<FileType>,
    current_dir: Option<PathBuf>,
    current_file: Option<PathBuf>,
}

pub struct BrowserStateBuilder {
    path: Option<PathBuf>,
    items: Vec<String>,
    file_type: FileType,
    current_dir: PathBuf,
    current_file: PathBuf,
}

fn read_dir(path: &PathBuf) -> Option<Vec<String>> {
    let reader = std::fs::read_dir(&path).ok()?;

    let mut items = reader
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>();

    items.retain(|item| !item.starts_with("."));

    if items.iter().all(|s| s.ends_with(".mp3")) {
        items = sort_by_track_number(items, path.to_path_buf())
    }

    Some(items)
}

#[memoize]
fn sort_by_track_number(song_names: Vec<String>, path: PathBuf) -> Vec<String> {
    let songs = song_names
        .iter()
        .map(|name| {
            SongBuilder::new()
                .from_path(&PathBuf::from(format!("{}/{}", path.display(), name)))
                .expect("REASON")
                .build()
        })
        .collect::<Vec<Song>>();

    let track_numbers: Vec<&u16> = songs
        .iter()
        .filter_map(|song| match song.track_number() {
            Ok(num) => Some(num),
            Err(_e) => None,
        })
        .collect();

    if song_names.len() != track_numbers.len() {
        return song_names;
    }

    let permutation = permutation::sort(&track_numbers);

    permutation
        .apply_slice(&songs)
        .iter()
        .map(|song| song.get_file_name())
        .collect()
}

impl BrowserStateBuilder {
    pub fn new() -> Self {
        BrowserStateBuilder {
            path: Some(PathBuf::new()),
            items: Vec::new(),
            file_type: FileType::NONE,
            current_dir: PathBuf::new(),
            current_file: PathBuf::new(),
        }
    }

    pub fn path(mut self, path: PathBuf) -> Result<Self> {
        self.items = read_dir(&path).expect("Error while reading dir");

        self.path = Some(path);

        Ok(self)
    }

    pub fn build(self) -> BrowserState {
        BrowserState {
            path: self.path.expect("BrowserStateBuilder: Path does not exist"),
            items: self.items,
            state: ListState::default(),
            file_type: Some(self.file_type),
            current_dir: Some(self.current_dir),
            current_file: Some(self.current_file),
        }
    }
}

impl BrowserState {
    // pub fn set_items(&mut self, items: Vec<String>) {
    //     self.items = items;
    //     self.state = ListState::default();
    // }

    pub fn update_state(&mut self) -> Result<()> {
        self.items = read_dir(&self.path).expect("Error while reading dir in update_state");

        let mut i = match self.state.selected() {
            Some(i) => i,
            None => 0,
        };

        if i > self.items.len() - 1 {
            i = 0;
        }

        let new_path = format!("{}/{}", &self.path.display(), &self.items[i]);

        let md = metadata(&new_path)?;

        if md.is_dir() {
            self.file_type = Some(FileType::DIRECTORY);
            self.current_dir = Some(self.items[i].clone().into());
            self.current_file = None;
        } else if md.is_file() {
            self.file_type = Some(FileType::FILE);
            self.current_dir = None;
            self.current_file = Some(self.items[i].clone().into());
        }

        Ok(())
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i));
        let _ = self.update_state();
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i));
        let _ = self.update_state();
    }

    pub fn select(&mut self) {
        if self.get_file_type() == FileType::DIRECTORY {
            match &self.get_current_dir() {
                Some(dir) => {
                    self.path.push(dir);
                    self.state.select(Some(0));
                }
                None => {}
            }
        }

        let _ = self.update_state();
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn get_current_dir(&self) -> Option<String> {
        match self.current_dir.clone() {
            Some(dir) => dir.into_os_string().into_string().ok(),
            None => None,
        }
    }

    pub fn get_current_file(&self) -> Option<String> {
        match self.current_file.clone() {
            Some(file) => file.into_os_string().into_string().ok(),
            None => None,
        }
    }

    pub fn get_file_type(&self) -> FileType {
        self.file_type.clone().expect("Nema razlog")
    }

    pub fn pop(&mut self) {
        self.path.pop();
        let _ = self.update_state();
    }
}

pub fn browser(app: &mut App, f: &mut Frame) -> Result<()> {
    let layout = Layout::default()
        .constraints(vec![Constraint::Min(1), Constraint::Max(1)])
        .split(f.size());

    f.render_stateful_widget(
        List::new(app.browser_state.items.clone())
            .highlight_symbol("$ ")
            .highlight_style(Style::default().bg(Color::DarkGray)),
        layout[0],
        &mut app.browser_state.state,
    );

    f.render_widget(
        Paragraph::new(
            app.browser_state
                .path
                .clone()
                .into_os_string()
                .into_string()
                .ok()
                .unwrap(),
        ),
        layout[1],
    );

    Ok(())
}
