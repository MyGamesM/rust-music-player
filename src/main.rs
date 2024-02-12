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

use ratatui::{prelude::*, widgets::Paragraph};

#[allow(dead_code)]
enum Screen {
    QUEUE,
    PLAYLISTS,
    BROWSER,
}

struct App {
    running: bool,
    queue: Queue,
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
    // let layout = Layout::default()
    //     .direction(Direction::Vertical)
    //     .constraints(vec![Constraint::Percentage(50)])
    //     .split(f.size());

    app.queue
        .next_clone()
        .unwrap()
        .tags()
        .iter()
        .enumerate()
        .for_each(|(i, tag)| {
            let area = Rect::new(0, i.try_into().unwrap(), f.size().width, 1);
            f.render_widget(Paragraph::new(tag.clone().unwrap()), area);
        });
}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('q') => app.running = false,
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

    let song = Song::new(path);

    q.add(song);

    // q.next_clone().unwrap().print();

    let mut app = App {
        running: true,
        queue: q,
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
