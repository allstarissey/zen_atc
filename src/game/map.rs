use serde::Deserialize;

use super::{
    object::Object,
    util::{Direction, Point},
};

const MINIMUM_WIDTH: u16 = 5;
const MINIMUM_HEIGHT: u16 = 5;

#[derive(Deserialize)]
pub struct Map {
    width: u16,
    height: u16,
    spawn_chance: f32,
    tick_rate: f32,
    objects: Vec<Object>,
    lines: Vec<Line>,
}

impl Map {
    fn label(&mut self) {
        let mut count_airports = 0;
        let mut count_beacons = 0;
        let mut count_exits = 0;

        for object in self.objects.iter_mut() {
            match object {
                Object::Airport { label, .. } => {
                    *label = Some(count_airports);
                    count_airports += 1;
                }
                Object::Beacon { label, .. } => {
                    *label = Some(count_beacons);
                    count_beacons += 1;
                }
                Object::Exit { label, .. } => {
                    *label = Some(count_exits);
                    count_exits += 1;
                }
            }
        }
    }

    fn validate_lines(&self) -> Result<(), MapError> {
        let Map { width, height, .. } = self;

        for line in self.lines.iter() {
            if !line.is_valid_slope() {
                return Err(MapError::Line(LineError::InvalidSlope(line)));
            }

            if !line.is_within_bounds(*width, *height) {
                return Err(MapError::Line(LineError::OutOfBounds(line)));
            }
        }

        Ok(())
    }

    fn validate_collisions(&self) -> Result<(), MapError> {
        for (k, object_a) in self.objects.iter().enumerate() {
            for object_b in self.objects.iter().skip(k + 1) {
                if object_a.position() == object_b.position() {
                    return Err(MapError::ObjectPlacement(
                        ObjectPlacementError::ConflictedSpace(object_a, object_b),
                    ));
                }
            }
        }

        Ok(())
    }

    fn validate_objects(&self) -> Result<(), MapError> {
        let (width, height) = (self.width, self.height);
        let (exits, objects): (Vec<&Object>, Vec<&Object>) = self
            .objects
            .iter()
            .partition(|o| matches!(o, Object::Exit { .. }));

        for object in objects {
            let Point(x, y) = object.position();

            if *x == 0 || *x > width || *y == 0 || *y > height {
                return Err(MapError::ObjectPlacement(
                    ObjectPlacementError::OutOfBounds(object),
                ));
            }
        }

        for exit in exits {
            let Point(x, y) = exit.position();

            let border_left = *x == 0;
            let border_top = *y == 0;
            let border_right = *x == width + 1;
            let border_bottom = *x == height + 1;

            if !(border_left || border_top || border_right || border_bottom) {
                return Err(MapError::ObjectPlacement(ObjectPlacementError::NotOnEdge(
                    exit,
                )));
            }

            let on_valid_edge = match exit.direction().unwrap() {
                Direction::North => border_top,
                Direction::Northeast => border_top && border_right,
                Direction::East => border_right,
                Direction::Southeast => border_bottom && border_right,
                Direction::South => border_bottom,
                Direction::Southwest => border_bottom && border_left,
                Direction::West => border_left,
                Direction::Northwest => border_top && border_left,
            };

            if !on_valid_edge {
                return Err(MapError::ObjectPlacement(
                    ObjectPlacementError::InvalidDirection(exit),
                ));
            }
        }

        Ok(())
    }

    fn validate(&self) -> Result<(), MapError> {
        if self.width < MINIMUM_WIDTH || self.height < MINIMUM_HEIGHT {
            return Err(MapError::InvalidSize(self.width, self.height));
        }

        if self.spawn_chance < 0.0 || self.spawn_chance > 1.0 {
            return Err(MapError::InvalidSpawnChance(self.spawn_chance));
        }

        self.validate_lines()?;
        self.validate_collisions()?;
        self.validate_objects()?;

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Line(Point, Point);

impl Line {
    fn is_valid_slope(&self) -> bool {
        let dx = self.0 .0.abs_diff(self.1 .0);
        let dy = self.0 .1.abs_diff(self.1 .1);

        match (dx, dy) {
            (0, 0) => false,
            (0, _) => true,
            (_, 0) => true,
            _ => dx == dy,
        }
    }

    fn is_within_bounds(&self, width: u16, height: u16) -> bool {
        let Point(x1, y1) = self.0;
        let Point(x2, y2) = self.1;

        x1 != 0
            && y1 != 0
            && x2 != 0
            && y2 != 0
            && x1 <= width
            && y1 <= height
            && x2 <= width
            && y2 <= height
    }
}

#[derive(Debug)]
enum MapError<'a> {
    InvalidSize(u16, u16),
    InvalidSpawnChance(f32),
    ObjectPlacement(ObjectPlacementError<'a>),
    Line(LineError<'a>),
}

impl std::fmt::Display for MapError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapError::InvalidSize(width, height) => {
                write!(f, "invalid map size: ({} x {})", width, height,)
            }
            MapError::InvalidSpawnChance(spawn_chance) => {
                write!(f, "invalid spawn chance: {}", spawn_chance,)
            }
            MapError::ObjectPlacement(e) => write!(f, "object placement error: {e}",),
            MapError::Line(e) => write!(f, "line error: {e}",),
        }
    }
}

impl std::error::Error for MapError<'_> {}

#[derive(Debug)]
enum ObjectPlacementError<'a> {
    OutOfBounds(&'a Object),
    NotOnEdge(&'a Object),
    InvalidDirection(&'a Object),
    ConflictedSpace(&'a Object, &'a Object),
}

impl std::fmt::Display for ObjectPlacementError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectPlacementError::ConflictedSpace(object_a, object_b) => write!(
                f,
                "{} {} and {} {} are on the same point",
                object_a.type_str(),
                object_a.label().unwrap(),
                object_b.type_str(),
                object_b.label().unwrap()
            ),
            ObjectPlacementError::InvalidDirection(object) => write!(
                f,
                "exit {} is facing an invalid direction",
                object.label().unwrap(),
            ),
            ObjectPlacementError::NotOnEdge(object) => {
                write!(f, "exit {} is not on edge", object.label().unwrap(),)
            }
            ObjectPlacementError::OutOfBounds(object) => write!(
                f,
                "{} {} is out of bounds",
                object.type_str(),
                object.label().unwrap(),
            ),
        }
    }
}

impl std::error::Error for ObjectPlacementError<'_> {}

#[derive(Debug)]
enum LineError<'a> {
    OutOfBounds(&'a Line),
    InvalidSlope(&'a Line),
}

impl std::fmt::Display for LineError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineError::InvalidSlope(line) => write!(
                f,
                "Line from ({}, {}) to ({}, {}) has an invalid slope",
                line.0 .0, line.0 .1, line.1 .0, line.1 .1
            ),
            LineError::OutOfBounds(line) => write!(
                f,
                "Line from ({}, {}) to ({}, {}) is out of bounds",
                line.0 .0, line.0 .1, line.1 .0, line.1 .1
            ),
        }
    }
}

impl std::error::Error for LineError<'_> {}
