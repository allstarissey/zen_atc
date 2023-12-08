mod command;
mod map;
mod object;
mod plane;
mod util;

use self::{command::Command, map::Map, plane::Plane};

pub struct App<'a> {
    map: Map,
    planes: Vec<Plane>,
    commands: Vec<Command<'a>>,
}
