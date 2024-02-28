#![allow(unused_imports)]
use crate::{App, PlaylistBuilder};
use anyhow::{bail, Result};
use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

pub fn test_ui(_app: &App, f: &mut Frame) -> Result<()> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(2), Constraint::Min(1)])
        .split(f.size());

    let playlist = match PlaylistBuilder::new().from_path("/mnt/hdd/Music/Albums/Bullet Hell II/") {
        Ok(playlist) => playlist.sort_by_track_number().build(),
        Err(_e) => bail!("Kita"),
    };

    playlist.songs().iter().enumerate().for_each(|(i, song)| {
        let area = Rect::new(0, i as u16 + 2, layout[0].width, 1);
        f.render_widget(
            Paragraph::new(format!("{}: {}", song.track_number(), song.title())),
            area,
        );
    });

    f.render_widget(Paragraph::new("Testing and debuging screen"), layout[0]);
    Ok(())
}

struct Events {
    items: Vec<String>,
    state: ListState,
}

#[allow(dead_code)]
impl Events {
    fn new(items: Vec<String>) -> Events {
        Events {
            items,
            state: ListState::default(),
        }
    }

    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
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
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

// pub fn test_list(_app: &App, f: &mut Frame) -> Result<()> {
//     let layout = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints(vec![Constraint::Min(1), Constraint::Max(2)])
//         .split(f.size());
//
//     let mut events = Events::new(vec![String::from("Item 1"), String::from("Item 2")]);
//
//     let items: Vec<ListItem> = events
//         .items
//         .iter()
//         .map(|i| ListItem::new(i.as_str()))
//         .collect();
//
//     let list = List::new(items)
//         .block(Block::default().title("List").borders(Borders::ALL))
//         .style(Style::default().fg(Color::White))
//         .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
//         .highlight_symbol(">>")
//         .repeat_highlight_symbol(true);
//
//     f.render_widget(list, layout[0]);
//
//     match events.state.selected() {
//         Some(text) => f.render_widget(Paragraph::new(format!("{}", text)), layout[1]),
//         None => f.render_widget(Paragraph::new("None"), layout[1]),
//     }
//
//     if event::poll(std::time::Duration::from_millis(250))? {
//         if let Key(key) = event::read()? {
//             if key.kind == event::KeyEventKind::Press {
//                 match key.code {
//                     Char('j') => events.next(),
//                     Char('k') => events.previous(),
//                     _ => {}
//                 }
//             }
//         }
//     }
//
//     Ok(())
// }
