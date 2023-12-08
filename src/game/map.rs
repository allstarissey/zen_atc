use std::time::Duration;

use super::{object::Object, util::Point};

const MINIMUM_WIDTH: u16 = 5;
const MINIMUM_HEIGHT: u16 = 5;

pub struct Map {
    width: u16,
    height: u16,
    spawn_chance: f32,
    tick_rate: Duration,
    objects: Vec<Object>,
    lines: Vec<Line>,
}

#[derive(Debug)]
pub struct Line(Point, Point);

#[derive(Debug)]
enum MapError<'a> {
    InvalidSize(u16, u16),
    InvalidSpawnChance(f32),
    ObjectPlacement(ObjectPlacementError<'a>),
    Line(LineError<'a>),
}

#[derive(Debug)]
enum ObjectPlacementError<'a> {
    OutOfBounds(&'a Object),
    NotOnEdge(&'a Object),
    InvalidDirection(&'a Object),
    ConflictedSpace(&'a Object, &'a Object),
}

#[derive(Debug)]
enum LineError<'a> {
    OutOfBounds(&'a Line),
    InvalidSlope(&'a Line),
}
