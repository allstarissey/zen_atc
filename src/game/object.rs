use serde::Deserialize;

use super::util::{Direction, Point};

#[derive(Debug, Deserialize)]
pub enum Object {
    Airport {
        position: Point,
        direction: Direction,

        #[serde(skip)]
        label: Option<u8>,
    },
    Beacon {
        position: Point,

        #[serde(skip)]
        label: Option<u8>,
    },
    Exit {
        position: Point,
        direction: Direction,

        #[serde(skip)]
        label: Option<u8>,
    },
}

impl Object {
    pub fn position(&self) -> &Point {
        match self {
            Object::Airport { position: pos, .. } => pos,
            Object::Beacon { position: pos, .. } => pos,
            Object::Exit { position: pos, .. } => pos,
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

    pub fn is_airport(&self) -> bool {
        matches!(self, Object::Airport { .. })
    }

    pub fn is_beacon(&self) -> bool {
        matches!(self, Object::Beacon { .. })
    }

    pub fn is_exit(&self) -> bool {
        matches!(self, Object::Exit { .. })
    }
}
