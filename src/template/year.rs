use crate::template::Day;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Year(u32);

pub fn year_day_string(year: Year, day: Day) -> String {
    format!("{year}::{day}")
}

impl Year {
    /// Creates a [`Year`] from the provided value
    /// returns [`None`] otherwise.
    pub const fn new(year: u32) -> Option<Self> {
        if year < 2015 { None } else { Some(Self(year)) }
    }

    /// Converts the [`Year`] into an [`u32`].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}", self.0)
    }
}

impl PartialEq<u32> for Year {
    fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<u32> for Year {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

/* -------------------------------------------------------------------------- */

impl FromStr for Year {
    type Err = YearFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let year = s.parse().map_err(|_| YearFromStrError)?;
        Self::new(year).ok_or(YearFromStrError)
    }
}

/// An error which can be returned when parsing a [`Year`].
#[derive(Debug)]
pub struct YearFromStrError;

impl Error for YearFromStrError {}

impl Display for YearFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("expecting a year >= 2015")
    }
}
