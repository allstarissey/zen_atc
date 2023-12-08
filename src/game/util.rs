#[derive(Debug, PartialEq)]
pub struct Point(pub u16, pub u16);

#[derive(Debug)]
pub enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}
