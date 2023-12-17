mod command;
mod map;
mod object;
mod plane;
mod util;

use std::{fs, path::PathBuf};

use self::{command::Command, map::Map, plane::Plane};

#[derive(Debug)]
pub struct App {
    map: Map,
    planes: Vec<Plane>,
    cur_command: CommandWriter,
}

impl App {
    pub fn new(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let map: Map = serde_json::from_str(fs::read_to_string(path)?.as_str())?;

        Ok(Self {
            map,
            planes: Vec::new(),
            cur_command: CommandWriter::new(),
        })
    }

    pub fn dimensions(&self) -> (&u16, &u16) {
        (self.map.width(), self.map.height())
    }
}
