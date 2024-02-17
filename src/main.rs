mod queue;
mod song;

use queue::Queue;
use song::Song;
use std::path::PathBuf;

use anyhow::Result;
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

fn ui(app: &App, f: &mut Frame) {
    match app.screen {
        Screen::ONE => ui1(app, f),
        Screen::TWO => ui2(app, f),
        _ => (),
    }
}

fn ui1(app: &App, f: &mut Frame) {
    // let items: Vec<String> = app.queue.next_clone().unwrap().tags();

    let items = std::fs::read_dir("/mnt/hdd/Music/Albums/Bullet Hell II/").unwrap();

    let borders = vec![
        Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT),
        Block::default().borders(Borders::LEFT | Borders::RIGHT),
        Block::default().borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT),
    ];

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

    let paths = items
        .map(|e| String::from(e.unwrap().path().to_str().unwrap()))
        .collect::<Vec<String>>();

    paths.iter().enumerate().for_each(|(i, path)| {
        let area = Rect::new(0, i as u16 + layout[0].height, layout[1].width, 1);

        let border = if i == 0 {
            borders[0].clone()
        } else if i == paths.len() - 1 {
            borders[2].clone()
        } else {
            borders[1].clone()
        };

        f.render_widget(
            Paragraph::new(String::from(
                path.strip_prefix("/mnt/hdd/Music/Albums/Bullet Hell II/")
                    .unwrap(),
            ))
            .block(border),
            area,
        )
    });

    f.render_widget(
        Paragraph::new(app.queue.next_clone().unwrap().tags()[0].clone())
            .block(Block::default().title("Now playing").borders(Borders::ALL)),
        layout[2],
    );
}

fn ui2(app: &App, f: &mut Frame) {
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

    let borders = vec![
        Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT),
        Block::default().borders(Borders::LEFT | Borders::RIGHT),
        Block::default().borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT),
    ];

    // let area = Rect::new(0, 5, f.size().width, 3);
    let middle = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .split(layout[1]);

    app.queue
        .next_clone()
        .unwrap()
        .tags()
        .iter()
        .enumerate()
        .for_each(|(i, tag)| {
            f.render_widget(
                Paragraph::new(Text::styled(tag.clone(), Style::new())).block(borders[i].clone()),
                middle[i],
            );
        });

    f.render_widget(
        Paragraph::new(app.queue.next_clone().unwrap().tags()[0].clone())
            .block(Block::default().title("Now playing").borders(Borders::ALL)),
        layout[2],
    );
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

    q.add(Song::new(path));

    let mut app = App {
        running: true,
        queue: q,
        screen: Screen::ONE,
    };

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    loop {
        update(&mut app)?;

        terminal.draw(|frame| {
            ui(&app, frame);
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
