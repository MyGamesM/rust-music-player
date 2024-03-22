use crate::App;
#[allow(unused_imports)]
use color_eyre::{
    eyre,
    eyre::{Report, Result},
};
#[allow(unused_imports)]
use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

fn center_layout(text_len: u16, layout: Rect) -> Rect {
    let padding = (layout.width - text_len) / 2;

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(Constraint::from_lengths([padding, text_len, padding]))
        .split(layout)[1]
}

pub fn screen_welcome(_app: &App, f: &mut Frame) -> Result<()> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Length(4),
            Constraint::Min(1),
            Constraint::Min(1),
        ])
        .split(f.size());

    let title = "Rust Music Player";

    f.render_widget(
        Paragraph::new(title).block(Block::new().borders(Borders::ALL)),
        center_layout(title.len() as u16, layout[0]),
    );

    f.render_widget(
        Paragraph::new("A simple cross platform tui music player written in rust."),
        center_layout(57, layout[1]),
    );

    let rows_controls = [
        Row::new(vec!["<p>", "Play or Pause current song"]),
        Row::new(vec!["<s>", "Skip current song"]),
        Row::new(vec!["<j>", "Move to next file"]),
        Row::new(vec!["<k>", "Move to previous line"]),
        Row::new(vec!["<l>", "Enter directory / Add song to queue"]),
        Row::new(vec!["<h>", "Go up a directory"]),
    ];

    let widths_controls = [Constraint::Length(3), Constraint::Length(35)];

    f.render_widget(
        Table::new(rows_controls, widths_controls),
        center_layout(38, layout[2]),
    );

    let rows_screens = [
        Row::new(vec!["<1>", "Welcome screen"]),
        Row::new(vec!["<4>", "Browser screen"]),
    ];

    let widths_screens = [Constraint::Length(3), Constraint::Length(14)];

    f.render_widget(
        Table::new(rows_screens, widths_screens),
        center_layout(18, layout[3]),
    );

    Ok(())
}
