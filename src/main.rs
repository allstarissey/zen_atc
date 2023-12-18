#![allow(dead_code)]

use std::{
    io,
    path::PathBuf,
    time::{Duration, Instant},
};

use clap::{Parser, Subcommand};
use crossterm::{
    event::{poll, read, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::App;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod game;
mod ui;

// todo: add real default path
const DEFAULT_MAP_PATH: &str = "test/map.json";
type BackendType = CrosstermBackend<io::Stdout>;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Play { path: Option<PathBuf> },
    List,
    Scores,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Play { path } => {
            let mut terminal = match instantiate_terminal() {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Error while instantiating terminal: {e}");
                    return;
                }
            };

            // ...
            fn sleep(duration: Duration) {
                let start = Instant::now();
                while start.elapsed() < duration {}
            }
            terminal.set_cursor(0, 0).unwrap();

            let default_path = PathBuf::from(DEFAULT_MAP_PATH);
            let path = path.unwrap_or(default_path);

            let app = match App::new(path) {
                Ok(a) => a,
                Err(e) => {
                    if let Err(e) = reset_terminal(terminal) {
                        eprintln!("Error while resetting terminal: {e}");
                    }
                    eprintln!("Error while loading map: {e}");
                    return;
                }
            };

            if let Err(e) = run_app(&mut terminal, app) {
                if let Err(e) = reset_terminal(terminal) {
                    eprintln!("Error while resetting terminal: {e}");
                }
                eprintln!("Error while running app: {e}");
                return;
            };

            if let Err(e) = reset_terminal(terminal) {
                eprintln!("Error while resetting terminal: {e}");
            }
        }
        Command::List => {}
        Command::Scores => {}
    }
}

fn instantiate_terminal() -> Result<Terminal<BackendType>, io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);

    Terminal::new(backend)
}

fn reset_terminal<B: Backend + io::Write>(mut terminal: Terminal<B>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend + io::Write>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut last_tick = Instant::now();
    let tick_duration = app.tick_duration();

    loop {
        terminal.draw(|f| ui::ui(f, &app))?;

        let poll_time = tick_duration - last_tick.elapsed();
        if poll(poll_time)? {
            if let Event::Key(key_event) = read()? {
                if app.handle_event(key_event) {
                    break;
                };
            }

            continue;
        }

        app.update();
        last_tick = Instant::now();
    }

    Ok(())
}
