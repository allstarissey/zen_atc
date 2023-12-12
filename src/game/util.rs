use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Point(pub u16, pub u16);

#[derive(Debug, Deserialize)]
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
