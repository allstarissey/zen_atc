use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Point(pub u16, pub u16);

pub const DIRECTION_CHARS: [char; 8] = ['q', 'w', 'e', 'a', 'd', 'z', 'x', 'c'];

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

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Direction::North => "north",
            Direction::Northeast => "northeast",
            Direction::East => "east",
            Direction::Southeast => "southeast",
            Direction::South => "south",
            Direction::Southwest => "southwest",
            Direction::West => "west",
            Direction::Northwest => "northwest",
        };

        write!(f, "{str}")
    }
}

#[derive(Debug)]
pub struct NoMatchError;
impl TryFrom<char> for Direction {
    type Error = NoMatchError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'w' => Ok(Direction::North),
            'e' => Ok(Direction::Northeast),
            'd' => Ok(Direction::East),
            'c' => Ok(Direction::Southeast),
            'x' => Ok(Direction::South),
            'z' => Ok(Direction::Southwest),
            'a' => Ok(Direction::West),
            'q' => Ok(Direction::Northwest),
            _ => Err(NoMatchError),
        }
    }
}
