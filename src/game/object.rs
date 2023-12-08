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
