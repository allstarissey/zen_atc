use super::{
    plane::{MarkStatus, Plane},
    util::Direction,
};

pub struct Command<'a> {
    command_type: CommandType,
    command_condition: Option<CommandCondition>,
    plane: &'a Plane,
}

pub enum CommandType {
    Climb(u16),
    Dive(u16),
    Turn(Direction),
    ChangeMark(MarkStatus),
}

pub enum CommandCondition {
    ArriveAirport(u8),
    ArriveBeacon(u8),
    Delay(u16),
}
