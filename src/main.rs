mod browser_list;
mod playlist;
mod queue;
mod song;

use browser_list::{BrowserState, BrowserStateBuilder, FileType};
use color_eyre::{
    eyre,
    eyre::{Report, Result},
};
use event::KeyCode;
use rodio::Sink;
use rodio::{Decoder, OutputStream};
use std::{fs::File, io, io::BufReader, panic, path::PathBuf, sync::mpsc, thread};
// use playlist::PlaylistBuilder;
// use song::Song;

// ratatui
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
    QUEUE,
    PLAYLISTS,
    BROWSER,
}

#[derive(PartialEq)]
enum PlayerState {
    PLAYING,
    PAUSED,
}

enum ThreadCommand {
    NONE,
    SONG,
    PLAYPAUSE,
    END,
    SKIP,
}

struct App {
    running: bool,
    browser_state: BrowserState,
    screen: Screen,
    tx: crate::mpsc::Sender<ThreadMessage>,
}

struct ThreadMessage {
    command: ThreadCommand,
    msg: Option<String>,
}

impl App {
    pub fn play_song(&self) -> Result<()> {
        let file = self
            .browser_state
            .get_current_file()
            .unwrap_or(String::from(""));

        let extensions = vec![".mp3", ".flac"];

        if file.is_empty() || !extensions.iter().any(|suffix| file.ends_with(suffix)) {
            return Ok(());
        }

        let path = String::from(format!(
            "{}/{}",
            self.browser_state
                .get_path()
                .into_os_string()
                .into_string()
                .ok()
                .unwrap(),
            file
        ));

        self.tx.send(ThreadMessage {
            command: ThreadCommand::SONG,
            msg: Some(path),
        })?;

        Ok(())
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
        Screen::BROWSER => Ok(browser_list::browser(app, f)?),
        _ => Ok(()),
    }
}

fn ui1(_app: &App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new("Rust Music Player").block(Block::default().borders(Borders::ALL)),
        f.size(),
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
}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('q') => app.running = false,
                    // change screens
                    Char('1') => app.screen = Screen::ONE,
                    Char('4') => app.screen = Screen::BROWSER,
                    // player controls
                    Char('p') => app.tx.send(ThreadMessage {
                        command: ThreadCommand::PLAYPAUSE,
                        msg: None,
                    })?,
                    Char('s') => app
                        .tx
                        .send(ThreadMessage {
                            command: ThreadCommand::SKIP,
                            msg: None,
                        })
                        .unwrap(),
                    _ => {}
                }

                if app.screen == Screen::BROWSER {
                    match key.code {
                        Char('j') => app.browser_state.next(),
                        Char('k') => app.browser_state.previous(),
                        Char('r') => app.browser_state.update_state()?,
                        KeyCode::Enter => match app.browser_state.get_file_type() {
                            FileType::FILE => {
                                app.play_song()?;
                            }
                            FileType::DIRECTORY => {
                                app.browser_state.select();
                            }
                            FileType::NONE => {}
                        },
                        KeyCode::Backspace => app.browser_state.pop(),
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}

fn init_player_thread(rx: std::sync::mpsc::Receiver<ThreadMessage>) -> Result<()> {
    thread::spawn(move || {
        let mut player_state = PlayerState::PAUSED;

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        loop {
            let message = match rx.recv() {
                Ok(message) => message,
                Err(_e) => ThreadMessage {
                    command: ThreadCommand::NONE,
                    msg: None,
                },
            };

            match message.command {
                ThreadCommand::NONE => {}
                ThreadCommand::PLAYPAUSE => match player_state {
                    PlayerState::PLAYING => {
                        sink.pause();
                        player_state = PlayerState::PAUSED;
                    }
                    PlayerState::PAUSED => {
                        sink.play();
                        player_state = PlayerState::PLAYING;
                    }
                },
                ThreadCommand::SONG => {
                    let file =
                        BufReader::new(File::open(PathBuf::from(message.msg.unwrap())).unwrap());

                    let source = Decoder::new(file).unwrap();

                    sink.append(source);
                    player_state = PlayerState::PLAYING;
                }
                ThreadCommand::SKIP => sink.skip_one(),
                ThreadCommand::END => break,
            }
        }
    });

    Ok(())
}

fn shutdown_player_thread(app: &App) -> Result<()> {
    app.tx
        .send(ThreadMessage {
            command: ThreadCommand::END,
            msg: None,
        })
        .unwrap();

    Ok(())
}

fn run() -> Result<()> {
    let (tx, rx) = mpsc::channel::<ThreadMessage>();

    init_player_thread(rx)?;

    let home_dir = match home::home_dir() {
        Some(path) => path,
        None => {
            return Err(Report::new(io::Error::new(
                io::ErrorKind::Other,
                "Could not find home directory!",
            )));
        }
    };

    // let path = PathBuf::from(env::current_dir()?);

    let browser_state = BrowserStateBuilder::new().path(home_dir)?.build();

    let mut app = App {
        running: true,
        browser_state,
        screen: Screen::BROWSER,
        tx,
    };

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    loop {
        update(&mut app)?;

        terminal.draw(|frame| {
            let _ = ui(&mut app, frame);
        })?;

        if !app.running {
            shutdown_player_thread(&app)?;
            break;
        }
    }

    Ok(())
}

pub fn install_hooks() -> Result<()> {
    // add any extra configuration you need to the hook builder
    let hook_builder = color_eyre::config::HookBuilder::default();
    let (panic_hook, eyre_hook) = hook_builder.into_hooks();

    // convert from a color_eyre PanicHook to a standard panic hook
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        shutdown().unwrap();
        panic_hook(panic_info);
    }));

    // convert from a color_eyre EyreHook to a eyre ErrorHook
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(move |error| {
        shutdown().unwrap();
        eyre_hook(error)
    }))?;

    Ok(())
}

fn main() -> Result<()> {
    startup()?;
    install_hooks()?;

    let result = run();

    shutdown()?;

    result?;

    Ok(())
}
