use super::util::{Direction, Point};

#[derive(Debug)]
pub struct Plane {
    position: Point,
    direction: Direction,
    mark_status: MarkStatus,
    label: char,
}

impl Plane {
    pub fn new(position: Point, direction: Direction, label: char) -> Self {
        Self {
            position,
            direction,
            label,
            mark_status: MarkStatus::Marked,
        }
    }

    pub fn label(&self) -> &char {
        &self.label
    }
}

#[derive(Debug)]
pub enum MarkStatus {
    Marked,
    Unmarked,
    Ignored,
}
