use std::str::FromStr;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReportLineError {
    #[error("invalid input")]
    InvalidInput,
}

#[derive(Debug, Clone)]
pub struct ReportLine(pub Vec<bool>);

impl FromStr for ReportLine {
    type Err = ReportLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ReportLine(
            s.chars()
                .map(|c| match c {
                    '0' => Ok(false),
                    '1' => Ok(true),
                    _ => Err(ReportLineError::InvalidInput),
                })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

pub struct ReportColumn(pub Vec<bool>);
