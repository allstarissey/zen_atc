#![allow(dead_code)]

use std::io;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod game;
mod ui;

type BackendType = CrosstermBackend<io::Stdout>;

fn main() {
    let terminal = match instantiate_terminal() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error while instantiating terminal: {e}");
            return;
        }
    };

    // ...

    if let Err(e) = reset_terminal(terminal) {
        eprintln!("Error while resetting terminal: {e}");
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
