use std::{
    io::{BufRead, stdin},
    ops::Deref,
    str::FromStr,
};

#[derive(Debug)]
enum Error {
    ParseDirection,
    ParseNumber,
    Io,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Mod100(u32);

impl Mod100 {
    const ZERO: Mod100 = Mod100(0);
}

impl From<u32> for Mod100 {
    fn from(value: u32) -> Self {
        Mod100(value % 100)
    }
}

impl std::ops::Add for Mod100 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Mod100::from(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Mod100 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Mod100::from(100 + self.0 - rhs.0)
    }
}

impl Deref for Mod100 {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Mod100 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u32 = s.parse().map_err(|_| Error::ParseNumber)?;
        Ok(Mod100::from(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Row {
    direction: Direction,
    count: Mod100,
}

impl FromStr for Row {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(Error::Io);
        }
        let (letter, count) = s.split_at(1);
        Ok(Row {
            direction: match letter {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => return Err(Error::ParseDirection),
            },
            count: count.parse()?,
        })
    }
}

fn main() {
    println!(
        "{}",
        stdin()
            .lock()
            .lines()
            .map(|v| v.map_err(|_| Error::Io).and_then(|v| v.parse::<Row>()))
            .fold((0, Mod100(50)), |(res, cursor), row| match row {
                Ok(Row {
                    direction: Direction::Left,
                    count,
                }) => match cursor - count {
                    Mod100::ZERO => (res + 1, 0.into()),
                    v => (res, v),
                },
                Ok(Row {
                    direction: Direction::Right,
                    count,
                }) => match cursor + count {
                    Mod100::ZERO => (res + 1, 0.into()),
                    v => (res, v),
                },
                _ => (res, cursor),
            })
            .0,
    )
}
