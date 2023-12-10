mod command;
mod map;
mod object;
mod plane;
mod util;

use std::{fs, path::PathBuf};

use self::{command::Command, map::Map, plane::Plane};

pub struct App<'a> {
    map: Map,
    planes: Vec<Plane>,
    commands: Vec<Command<'a>>,
    cur_command: String,
}

impl App<'_> {
    pub fn new(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let map: Map = serde_json::from_str(fs::read_to_string(path)?.as_str())?;

        Ok(Self {
            map,
            planes: Vec::new(),
            commands: Vec::new(),
            cur_command: String::new(),
        })
    }

    pub fn dimensions(&self) -> (&u16, &u16) {
        (self.map.width(), self.map.height())
    }
}
