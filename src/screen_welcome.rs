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

fn center_text_layout(text_len: u16, layout: Rect) -> Rect {
    let padding = (layout.width - text_len) / 2;

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(Constraint::from_lengths([padding, text_len, padding]))
        .split(layout)[1]
}

pub fn screen_welcome(_app: &App, f: &mut Frame) -> Result<()> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(3), Constraint::Max(4)])
        .split(f.size());

    let title = "Rust Music Player";

    f.render_widget(
        Paragraph::new(title).block(Block::new().borders(Borders::ALL)),
        center_text_layout(title.len() as u16, layout[0]),
    );

    f.render_widget(
        Paragraph::new("A cross platform tui music player written in rust"),
        center_text_layout(49, layout[1]),
    );

    Ok(())
}
