use crate::{command::Command, direction::Direction};

pub struct Position {
    pub horizontal: i32,
    pub depth: i32,
    pub aim: i32,
}

impl Position {
    pub fn apply_command(&self, command: &Command) -> Position {
        let Command {
            direction,
            distance,
        } = command;

        match direction {
            Direction::Forward => Position {
                horizontal: self.horizontal + distance,
                depth: self.depth + (distance * self.aim),
                aim: self.aim,
            },
            Direction::Down => Position {
                aim: self.aim + distance,
                ..*self
            },
            Direction::Up => Position {
                aim: self.aim - distance,
                ..*self
            },
        }
    }
}
