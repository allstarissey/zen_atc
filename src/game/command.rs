use super::{
    object::Object,
    plane::{MarkStatus, Plane},
    util::Direction,
};

#[derive(Debug)]
pub struct Command<'a> {
    command_type: CommandType,
    command_condition: Option<CommandCondition<'a>>,
    plane: &'a Plane,
}

#[derive(Debug)]
pub enum CommandType {
    Climb(u8),
    Dive(u8),
    Turn(Direction),
    ChangeMark(MarkStatus),
}

#[derive(Debug)]
pub enum CommandCondition<'a> {
    ArriveAirport(&'a Object),
    ArriveBeacon(&'a Object),
    Delay(u8),
}

#[derive(Debug)]
pub struct CommandWriter {
    cur_string: String,
}

impl CommandWriter {
    pub fn new() -> Self {
        Self {
            cur_string: String::new(),
        }
    }

    pub fn push(&mut self, input: char) {
        if self.cur_string.is_empty() && input.is_alphabetic() {
            self.cur_string.push(input);
            return;
        }

        let prev_char = self.cur_string.chars().last().unwrap();
        match self.cur_string.len() {
            1 => match input {
                't' | 'c' | 'd' => self.cur_string.push(input),
                'm' | 'u' | 'i' => {
                    self.cur_string.push(input);
                    self.cur_string.push('_');
                }
                _ => (),
            },
            2 => {
                if matches!(
                    (prev_char, input),
                    ('t', 'q' | 'w' | 'e' | 'a' | 'd' | 'z' | 'x' | 'c') | ('c' | 'd', '0'..='9')
                ) {
                    self.cur_string.push(input)
                }
            }
            3 => {
                if matches!(input, 'a' | 'i') {
                    self.cur_string.push(input)
                }
            }
            4 => {
                if matches!((prev_char, input), ('i', '0'..='9') | ('a', 'a' | 'b')) {
                    self.cur_string.push(input)
                }
            }
            5 => {
                if matches!((prev_char, input), ('a' | 'b' | '0'..='9', '0'..='9')) {
                    self.cur_string.push(input)
                }
            }
            _ => (),
        }
    }

    pub fn pop(&mut self) {
        self.cur_string.pop();
    }

    pub fn clear(&mut self) {
        self.cur_string.clear();
    }

    pub fn build<'a>(&'a self, planes: &'a [Plane], objects: &'a [Object]) -> Option<Command> {
        if self.cur_string.is_empty() {
            return None;
        }

        let mut string_iter = self.cur_string.chars();

        let plane_char = string_iter.next().unwrap();
        let plane = planes.iter().find(|p| p.label() == &plane_char)?;

        let command_type_chars: [char; 2] = string_iter
            .by_ref()
            .take(2)
            .collect::<Vec<char>>()
            .try_into()
            .ok()?;
        let command_type = match command_type_chars {
            ['t', dir] => Some(CommandType::Turn(Direction::try_from(dir).ok()?)),
            ['c', alt] => Some(CommandType::Climb(to_digit(alt)?)),
            ['d', alt] => Some(CommandType::Dive(to_digit(alt)?)),
            ['m', _] => Some(CommandType::ChangeMark(MarkStatus::Marked)),
            ['u', _] => Some(CommandType::ChangeMark(MarkStatus::Unmarked)),
            ['i', _] => Some(CommandType::ChangeMark(MarkStatus::Ignored)),
            _ => None,
        }?;

        let condition_chars_vec: Vec<char> = string_iter.collect();
        if condition_chars_vec.is_empty() {
            return Some(Command {
                command_type,
                command_condition: None,
                plane,
            });
        }

        let condition_chars: [char; 3] = condition_chars_vec.try_into().ok()?;
        let command_condition = match condition_chars {
            ['a', 'a', label] => {
                let label = to_digit(label)?;
                let airport = objects.iter().find(|o| {
                    o.is_airport() && o.label().unwrap() == &label
                })?;

                Some(CommandCondition::ArriveAirport(airport))
            }
            ['a', 'b', label] => {
                let label = to_digit(label)?;
                let beacon = objects.iter().find(|o| {
                    o.is_beacon() && o.label().unwrap() == &label
                })?;

                Some(CommandCondition::ArriveBeacon(beacon))
            }
            ['i', digit_tens, digit_ones] => Some(CommandCondition::Delay(
                to_digit(digit_tens)? * 10u8 + to_digit(digit_ones)?,
            )),
            _ => None,
        };

        Some(Command {
            command_type,
            command_condition,
            plane,
        })
    }
}

fn to_digit(ch: char) -> Option<u8> {
    Some(ch.to_digit(10)? as u8)
}
