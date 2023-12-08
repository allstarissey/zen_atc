use super::util::{Direction, Point};

pub struct Plane {
    position: Point,
    direction: Direction,
    mark_status: MarkStatus,
    label: char,
}

pub enum MarkStatus {
    Marked,
    Unmarked,
    Ignored,
}
