mod playlist;
mod queue;
mod song;

use playlist::PlaylistBuilder;
use queue::Queue;
use song::Song;
use std::path::PathBuf;

use anyhow::{bail, Result};
use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

#[allow(dead_code)]
enum Screen {
    ONE,
    TWO,
    QUEUE,
    PLAYLISTS,
    BROWSER,
}

struct App {
    running: bool,
    queue: Queue,
    screen: Screen,
}

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(app: &App, f: &mut Frame) -> Result<()> {
    match app.screen {
        Screen::ONE => Ok(ui1(app, f)),
        Screen::TWO => Ok(ui2(app, f)?),
        _ => Ok(()),
    }
}

fn ui1(app: &App, f: &mut Frame) {
    let items = std::fs::read_dir("/mnt/hdd/Music/Albums/Bullet Hell II/").unwrap();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Max(3),
            Constraint::Min(1),
            Constraint::Max(3),
        ])
        .split(f.size());

    f.render_widget(
        Paragraph::new("Rust Music Player").block(Block::default().borders(Borders::ALL)),
        layout[0],
    );

    let items = items
        .map(|e| {
            let song_name = String::from(e.unwrap().path().to_str().unwrap());

            match song_name.strip_prefix("/mnt/hdd/Music/Albums/Bullet Hell II/") {
                Some(s) => return String::from(s),
                None => return String::from("Kita"),
            };
        })
        .collect::<Vec<String>>();

    f.render_widget(
        // selecting??
        List::new(items)
            .block(Block::default().title("List").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol("$")
            .repeat_highlight_symbol(true),
        layout[1],
    );

    f.render_widget(
        Paragraph::new(app.queue.next_clone().unwrap().tags()[0].clone())
            .block(Block::default().title("Now playing").borders(Borders::ALL)),
        layout[2],
    );
}

fn ui2(app: &App, f: &mut Frame) -> Result<()> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Max(3),
            Constraint::Min(1),
            Constraint::Max(3),
        ])
        .split(f.size());

    f.render_widget(
        Paragraph::new("Kita").block(Block::default().borders(Borders::ALL)),
        layout[0],
    );

    // let borders = vec![
    //     Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT),
    //     Block::default().borders(Borders::LEFT | Borders::RIGHT),
    //     Block::default().borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT),
    // ];

    app.queue
        .next_clone()
        .unwrap()
        .tags()
        .iter()
        .enumerate()
        .for_each(|(i, tag)| {
            let area = Rect::new(0, i as u16 + layout[0].height, layout[1].width, 1);

            f.render_widget(
                Paragraph::new(Text::styled(tag.clone(), Style::new())),
                // .block(border.clone()),
                area,
            );
        });

    f.render_widget(Paragraph::new(""), layout[1]);

    let _playlist = match PlaylistBuilder::new().from_path("/mnt/hdd/Music/Albums/Bullet Hell II/")
    {
        Ok(playlist) => playlist.build(),
        Err(_e) => bail!("Kita"),
    };

    f.render_widget(
        Paragraph::new(app.queue.next_clone().unwrap().tags()[0].clone())
            .block(Block::default().title("Now playing").borders(Borders::ALL)),
        layout[2],
    );

    Ok(())
}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('q') => app.running = false,
                    Char('1') => app.screen = Screen::ONE,
                    Char('2') => app.screen = Screen::TWO,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    let mut q: Queue = Queue {
        queue: vec![],
        playing: 0,
    };

    let path = PathBuf::from(
        "/mnt/hdd/Music/Albums/Bullet Hell II/RichaadEB - Emotional Skyscraper ~ Cosmic Mind.mp3",
    );

    q.add(Song::new().from_path(&path)?.build());

    let mut app = App {
        running: true,
        queue: q,
        screen: Screen::ONE,
    };

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    loop {
        update(&mut app)?;

        terminal.draw(|frame| {
            let _ = ui(&app, frame);
        })?;

        if !app.running {
            break;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    startup()?;

    let result = run();

    shutdown()?;

    result?;

    Ok(())
}
