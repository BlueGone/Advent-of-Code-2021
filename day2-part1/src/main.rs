mod command;
mod direction;
mod position;

use crate::{command::Command, position::Position};

const INPUT: &str = include_str!("./input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let commands = INPUT
        .lines()
        .map(|line| line.parse::<Command>())
        .collect::<Result<Vec<_>, _>>()?;

    let position = commands.iter().fold(
        Position {
            horizontal: 0,
            depth: 0,
        },
        |position, command| position.apply_command(command),
    );

    println!("{}", position.horizontal * position.depth);

    Ok(())
}
