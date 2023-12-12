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
                if matches!((prev_char, input), ('a' | 'b', '0'..='9')) {
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
}
