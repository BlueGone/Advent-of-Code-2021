use crate::{command::Command, direction::Direction};

pub struct Position {
    pub horizontal: i32,
    pub depth: i32,
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
                depth: self.depth,
            },
            Direction::Down => Position {
                horizontal: self.horizontal,
                depth: self.depth + distance,
            },
            Direction::Up => Position {
                horizontal: self.horizontal,
                depth: self.depth - distance,
            },
        }
    }
}
