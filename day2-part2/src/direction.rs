use std::str::FromStr;

use thiserror::Error;

#[derive(Error, Debug)]
#[error("Could not parse '{str}' as a direction")]
pub struct ParseDirectionError {
    str: String,
}

pub enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(ParseDirectionError { str: s.to_string() }),
        }
    }
}
