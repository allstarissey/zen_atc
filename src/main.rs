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
    use game::App;
    let mut terminal = match instantiate_terminal() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error while instantiating terminal: {e}");
            return;
        }
    };

    // ...
    use std::time::{Duration, Instant};
    fn sleep(dur: Duration) {
        let start = Instant::now();
        while start.elapsed() < dur {}
    }

    terminal.set_cursor(0, 0).unwrap();

    let path = std::path::PathBuf::from("/mnt/dev/Rust/zen_atc/test/map.json");
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

    terminal.draw(|f| ui::ui(f, &app)).unwrap();

    sleep(Duration::from_secs(5));
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
