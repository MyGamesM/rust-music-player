mod browser_list;
mod playlist;
mod queue;
mod song;
mod test_ui;

use browser_list::{BrowserState, BrowserStateBuilder};
use event::KeyCode;
use playlist::PlaylistBuilder;
use queue::Queue;
use rodio::Sink;
use rodio::{Decoder, OutputStream};
// use song::Song;
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::{env, path::PathBuf};

use anyhow::Result;
use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

#[allow(dead_code)]
#[derive(PartialEq)]
enum Screen {
    ONE,
    TWO,
    QUEUE,
    PLAYLISTS,
    BROWSER,
    TEST,
}

struct App {
    running: bool,
    queue: Queue,
    screen: Screen,
    browser_state: BrowserState,
    handles: Vec<std::thread::JoinHandle<()>>,
}

impl App {
    pub fn play_song(&mut self) {
        let path = String::from(format!(
            "{}/{}",
            self.browser_state
                .get_path()
                .into_os_string()
                .into_string()
                .ok()
                .unwrap(),
            self.browser_state.get_current_file().unwrap()
        ));

        self.handles.push(thread::spawn(move || {
            // Get a output stream handle to the default physical sound device
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            // Load a sound from a file, using a path relative to Cargo.toml
            let file = BufReader::new(File::open(&path).unwrap());
            // Decode that sound file into a source
            let source = Decoder::new(file).unwrap();
            // Play the sound directly on the device
            let sink = Sink::try_new(&stream_handle).unwrap();

            // make this run on a seperate thread
            sink.append(source);
            sink.sleep_until_end();
        }))

        // The sound plays in a separate audio thread,
        // so we need to keep the main thread alive while it's playing.
        // std::thread::sleep(std::time::Duration::from_secs(song_duration));
    }
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

fn ui(app: &mut App, f: &mut Frame) -> Result<()> {
    match app.screen {
        Screen::ONE => Ok(ui1(app, f)),
        Screen::TWO => Ok(ui2(app, f)?),
        Screen::TEST => Ok(test_ui::test_ui(app, f)?),
        Screen::BROWSER => Ok(browser_list::browser(app, f)?),
        _ => Ok(()),
    }
}

fn ui1(_app: &App, f: &mut Frame) {
    // let items = std::fs::read_dir("/mnt/hdd/Music/Albums/Bullet Hell II/").unwrap();

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

    // let items = items
    //     .map(|e| {
    //         let song_name = String::from(e.unwrap().path().to_str().unwrap());
    //
    //         match song_name.strip_prefix("/mnt/hdd/Music/Albums/Bullet Hell II/") {
    //             Some(s) => String::from(s),
    //             None => String::from("Kita"),
    //         }
    //     })
    //     .collect::<Vec<String>>();
    //
    // f.render_widget(
    //     // selecting??
    //     List::new(items)
    //         .block(Block::default().title("List").borders(Borders::ALL))
    //         .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    //         .highlight_symbol("$")
    //         .repeat_highlight_symbol(true),
    //     layout[1],
    // );

    // f.render_widget(
    //     Paragraph::new(app.queue.next_clone().unwrap().tags()[0].clone())
    //         .block(Block::default().title("Now playing").borders(Borders::ALL)),
    //     layout[2],
    // );
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
                    Char('4') => app.screen = Screen::BROWSER,
                    Char('0') => app.screen = Screen::TEST,
                    // browser
                    _ => {}
                }

                if app.screen == Screen::BROWSER {
                    match key.code {
                        Char('j') => app.browser_state.next(),
                        Char('k') => app.browser_state.previous(),
                        Char('p') => app.play_song(),
                        Char('r') => app.browser_state.update_state()?,
                        KeyCode::Enter => app.browser_state.select(),
                        KeyCode::Backspace => app.browser_state.pop(),
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    let q: Queue = Queue {
        queue: vec![],
        playing: 0,
    };

    // let path = PathBuf::from("/mnt/hdd/Music/Albums/Bullet Hell II/");

    let path = PathBuf::from(env::current_dir()?);

    // q.add(Song::new().from_path(&path)?.build());

    let browser_state = BrowserStateBuilder::new().path(path)?.build();

    let handles = vec![];

    let mut app = App {
        running: true,
        queue: q,
        screen: Screen::BROWSER,
        browser_state,
        handles,
    };

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    loop {
        update(&mut app)?;

        terminal.draw(|frame| {
            let _ = ui(&mut app, frame);
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
