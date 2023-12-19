use super::{
    object::Object,
    plane::{MarkStatus, Plane},
    util::Direction,
};

#[derive(Debug)]
pub struct Command {
    command_type: CommandType,
    command_condition: Option<CommandCondition>,
    tick: u32,
}

impl Command {
    fn new(
        command_type: CommandType,
        command_condition: Option<CommandCondition>,
        tick: u32,
    ) -> Self {
        Self {
            command_type,
            command_condition,
            tick,
        }
    }

    pub fn tick(&self) -> &u32 {
        &self.tick
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub enum CommandType {
    Climb(u8),
    Dive(u8),
    Turn(Direction),
    ChangeMark(MarkStatus),
}

#[derive(Debug)]
pub enum CommandCondition {
    ArriveAirport(u8),
    ArriveBeacon(u8),
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
        if let Some('_') = self.cur_string.pop() {
            self.cur_string.pop();
        }
    }

    pub fn clear(&mut self) {
        self.cur_string.clear();
    }

    //? Change Option<Command> to Result<Command, CommandBuildError>
    pub fn build<'a>(
        self,
        planes: &'a [Plane],
        objects: &'a [Object],
        tick: u32,
    ) -> Option<(Command, char)> {
        if self.cur_string.is_empty() {
            return None;
        }

        let mut string_iter = self.cur_string.chars();

        let plane = string_iter.next().unwrap();
        if !planes.iter().any(|p| p.label() == &plane) {
            return None;
        }

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
            return Some((Command::new(command_type, None, tick), plane));
        }

        let condition_chars: [char; 3] = condition_chars_vec.try_into().ok()?;
        let command_condition = match condition_chars {
            ['a', 'a', label] => {
                let airport = to_digit(label)?;
                if !objects
                    .iter()
                    .any(|o| o.is_airport() && o.label().unwrap() == &airport)
                {
                    return None;
                }

                Some(CommandCondition::ArriveAirport(airport))
            }
            ['a', 'b', label] => {
                let beacon = to_digit(label)?;
                if !objects
                    .iter()
                    .any(|o| o.is_beacon() && o.label().unwrap() == &beacon)
                {
                    return None;
                }

                Some(CommandCondition::ArriveBeacon(beacon))
            }
            ['i', digit_tens, digit_ones] => Some(CommandCondition::Delay(
                to_digit(digit_tens)? * 10u8 + to_digit(digit_ones)?,
            )),
            _ => None,
        };

        Some((Command::new(command_type, command_condition, tick), plane))
    }
}

impl Default for CommandWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for CommandWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_string = String::new();
        let mut chars = self.cur_string.chars();

        let plane = match chars.next() {
            Some(p) => format!("{p}:"),
            None => return Ok(()),
        };
        display_string.push_str(&plane);

        let command_type = match chars.next() {
            Some('c') => " climb",
            Some('d') => " dive",
            Some('t') => " turn",
            Some('m') => " mark",
            Some('u') => " unmark",
            Some('i') => " ignore",
            Some(x) => panic!("Invalid command character encountered: {x}"),
            None => return write!(f, "{display_string}"),
        };
        display_string.push_str(command_type);

        let command_arg = match chars.next() {
            Some('_') => "".to_owned(),
            Some(ch) if is_direction(ch) => format!(" {}", Direction::try_from(ch).unwrap()),
            Some(num) if num.is_numeric() => format!(" {num}000 feet"),
            Some(x) => panic!("Invalid command argument encountered: {x}"),
            None => return write!(f, "{display_string}"),
        };
        display_string.push_str(&command_arg);

        let condition_type = match chars.next() {
            Some('i') => " in",
            Some('a') => " at",
            Some(x) => panic!("Invalid condition type encountered: {x}"),
            None => return write!(f, "{display_string}"),
        };
        display_string.push_str(condition_type);

        let is_delay: bool;
        let condition_arg_1 = match chars.next() {
            Some('a') => {
                is_delay = false;
                " airport:".to_owned()
            }
            Some('b') => {
                is_delay = false;
                " beacon:".to_owned()
            }
            Some(num) if num.is_numeric() => {
                is_delay = true;
                format!(" {num}")
            }
            Some(x) => panic!("Invalid condition argument encountered: {x}"),
            None => return write!(f, "{display_string}"),
        };
        display_string.push_str(&condition_arg_1);

        let condition_arg_2 = match chars.next() {
            Some(num) if num.is_numeric() => {
                if is_delay {
                    format!("{num} seconds")
                } else {
                    format!(" {num}")
                }
            }
            Some(x) => panic!("Invalid condition argument encountered: {x}"),
            None => return write!(f, "{display_string}"),
        };
        display_string.push_str(&condition_arg_2);

        write!(f, "{display_string}")
    }
}

fn to_digit(ch: char) -> Option<u8> {
    Some(ch.to_digit(10)? as u8)
}

fn is_direction(ch: char) -> bool {
    use super::util::DIRECTION_CHARS;
    if !ch.is_alphabetic() {
        return false;
    }
    DIRECTION_CHARS.iter().any(|c| c == &ch)
}
