mod command;
mod map;
mod object;
mod plane;
mod util;

use std::{fs, path::PathBuf, time::Duration};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use self::{command::CommandWriter, map::Map, plane::Plane};

#[derive(Debug)]
pub struct App {
    map: Map,
    planes: Vec<Plane>,
    cur_command: CommandWriter,
    tick: u32,
}

impl App {
    pub fn new(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let map: Map = serde_json::from_str(fs::read_to_string(path)?.as_str())?;

        Ok(Self {
            map,
            planes: Vec::new(),
            cur_command: CommandWriter::new(),
            tick: 0,
        })
    }

    pub fn dimensions(&self) -> (&u16, &u16) {
        (self.map.width(), self.map.height())
    }

    pub fn tick_duration(&self) -> Duration {
        Duration::from_secs_f32(*self.map.tick_rate())
    }

    pub fn cur_command(&self) -> String {
        self.cur_command.to_string()
    }

    pub fn tick(&mut self) {
        // todo!()
    }

    fn build_command(&mut self) {
        let cur_command = std::mem::take(&mut self.cur_command);
        let (command, plane) = match cur_command.build(&self.planes, self.map.objects(), self.tick) {
            Some(c) => c,
            None => return,
        };

        self.planes
            .iter_mut()
            .find(|p| p.label() == &plane)
            .unwrap()
            .push_command(command);
    }

    pub fn handle_event(&mut self, key_event: KeyEvent) -> bool {
        match key_event {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => return true,
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            }
            | KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => self.cur_command.pop(),
            KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::NONE,
                ..
            } => self.cur_command.push(ch),
            KeyEvent {
                code: KeyCode::Esc, ..
            } => self.cur_command.clear(),
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => self.build_command(),
            _ => (),
        }

        false
    }
}
