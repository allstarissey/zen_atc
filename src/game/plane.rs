use super::{
    command::Command,
    util::{Direction, Point},
};

#[derive(Debug)]
pub struct Plane {
    label: char,
    position: Point,
    direction: Direction,
    mark_status: MarkStatus,
    commands: Vec<Command>,
}

impl Plane {
    pub fn new(position: Point, direction: Direction, label: char) -> Self {
        Self {
            label,
            position,
            direction,
            mark_status: MarkStatus::Marked,
            commands: Vec::new(),
        }
    }

    pub fn label(&self) -> &char {
        &self.label
    }

    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    pub fn push_command(&mut self, command: Command) {
        self.commands.push(command);
    }
}

#[derive(Debug)]
pub enum MarkStatus {
    Marked,
    Unmarked,
    Ignored,
}
