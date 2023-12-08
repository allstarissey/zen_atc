use super::util::{Direction, Point};

#[derive(Debug)]
pub enum Object {
    Airport {
        pos: Point,
        direction: Direction,
        label: Option<u8>,
    },
    Beacon {
        pos: Point,
        label: Option<u8>,
    },
    Exit {
        pos: Point,
        direction: Direction,
        label: Option<u8>,
    },
}

impl Object {
    pub fn pos(&self) -> &Point {
        match self {
            Object::Airport { pos, .. } => pos,
            Object::Beacon { pos, .. } => pos,
            Object::Exit { pos, .. } => pos,
        }
    }

    pub fn direction(&self) -> Option<&Direction> {
        match self {
            Object::Airport { direction, .. } => Some(direction),
            Object::Beacon { .. } => None,
            Object::Exit { direction, .. } => Some(direction),
        }
    }

    pub fn label(&self) -> Option<&u8> {
        match self {
            Object::Airport { label, .. } => label.as_ref(),
            Object::Beacon { label, .. } => label.as_ref(),
            Object::Exit { label, .. } => label.as_ref(),
        }
    }

    pub fn type_str(&self) -> String {
        match self {
            Object::Airport { .. } => "airport".to_owned(),
            Object::Beacon { .. } => "beacon".to_owned(),
            Object::Exit { .. } => "exit".to_owned(),
        }
    }
}