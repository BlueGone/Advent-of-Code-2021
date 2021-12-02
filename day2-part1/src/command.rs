use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

use crate::direction::{Direction, ParseDirectionError};

#[derive(Error, Debug)]
pub enum ParseCommandError {
    #[error("Invalid word count for a command")]
    WordCount,
    #[error(transparent)]
    Distance(#[from] ParseIntError),
    #[error(transparent)]
    Direction(#[from] ParseDirectionError),
}

pub struct Command {
    pub direction: Direction,
    pub distance: i32,
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(' ').collect::<Vec<_>>();
        let [direction, distance] = match words.as_slice() {
            [direction, distance] => Ok([direction, distance]),
            _ => Err(ParseCommandError::WordCount),
        }?;

        let distance = distance
            .parse::<i32>()
            .map_err(ParseCommandError::Distance)?;
        let direction = direction
            .parse::<Direction>()
            .map_err(ParseCommandError::Direction)?;

        Ok(Command {
            direction,
            distance,
        })
    }
}
