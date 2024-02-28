use crate::App;
use anyhow::{bail, Ok, Result};
use ratatui::{prelude::*, widgets::*};
use std::fs::metadata;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum FileType {
    FILE,
    DIRECTORY,
    NONE,
}

#[allow(dead_code)]
pub struct BrowserState {
    path: PathBuf,
    items: Vec<String>,
    state: ListState,
    file_type: Option<FileType>,
    current_dir: Option<PathBuf>,
    current_file: Option<PathBuf>,
}

#[allow(dead_code)]
pub struct BrowserStateBuilder {
    path: Option<PathBuf>,
    items: Vec<String>,
    state: ListState,
    file_type: FileType,
    current_dir: PathBuf,
    current_file: PathBuf,
}

impl BrowserStateBuilder {
    pub fn new() -> Self {
        BrowserStateBuilder {
            path: Some(PathBuf::new()),
            items: Vec::new(),
            state: ListState::default(),
            file_type: FileType::NONE,
            current_dir: PathBuf::new(),
            current_file: PathBuf::new(),
        }
    }

    pub fn path(mut self, path: PathBuf) -> Result<Self> {
        let items = match std::fs::read_dir(&path) {
            std::result::Result::Ok(files) => files,
            Err(e) => bail!(format!("Error reading path {}", e)),
        };

        self.path = Some(path);

        self.items = items
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    e.path()
                        .file_name()
                        .and_then(|n| n.to_str().map(|s| String::from(s)))
                })
            })
            .collect::<Vec<String>>();

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

#[allow(dead_code)]
impl BrowserState {
    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
        self.state = ListState::default();
    }

    pub fn update_state(&mut self) -> Result<()> {
        let items = match std::fs::read_dir(&self.path) {
            std::result::Result::Ok(files) => files,
            Err(e) => bail!(format!("Error reading path {}", e)),
        };

        let mut i = match self.state.selected() {
            Some(i) => i,
            None => 0,
        };

        // let i = 0;

        self.items = items
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    e.path()
                        .file_name()
                        .and_then(|n| n.to_str().map(|s| String::from(s)))
                })
            })
            .collect::<Vec<String>>();

        if i > self.items.len() - 1 {
            i = 0;
        }

        let new_path = format!("{}/{}", &self.path.display(), &self.items[i]);

        let md = match metadata(&new_path) {
            std::result::Result::Ok(md) => md,
            Err(e) => bail!(format!("Error reading file metadata: {}\n{}", e, new_path)),
        };

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
                Some(dir) => self.path.push(dir),
                None => {}
            }
        }

        if self.get_file_type() == FileType::FILE {
            let new_path = PathBuf::from(format!(
                "{}/{}",
                &self.path.display(),
                &self.get_current_file().unwrap()
            ));

            match &new_path.extension() {
                Some(ext) => println!("{:?}", ext.to_str().or(Some("None2"))),
                None => println!("{:?}", &new_path),
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
            .highlight_symbol(">>")
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
