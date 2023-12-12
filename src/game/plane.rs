use super::util::{Direction, Point};

#[derive(Debug)]
pub struct Plane {
    position: Point,
    direction: Direction,
    mark_status: MarkStatus,
    label: char,
}

impl Plane {
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
