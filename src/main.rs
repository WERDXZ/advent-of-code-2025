use std::{
    io::{stdin, BufRead},
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
struct Mod100 {
    pub value: i32,
    pub wrap: u32,
}

impl From<i32> for Mod100 {
    fn from(value: i32) -> Self {
        Mod100 {
            value: value % 100,
            wrap: (value / 100).unsigned_abs(),
        }
    }
}

impl std::ops::Add for Mod100 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.value == 0 {
            Mod100 {
                value: rhs.value.rem_euclid(100),
                wrap: self.wrap + rhs.wrap,
            }
        } else {
            let val = self.value + rhs.value;
            match val {
                v if !(1..100).contains(&v) => Mod100 {
                    value: v.rem_euclid(100),
                    wrap: self.wrap + rhs.wrap + 1,
                },
                _ => Mod100 {
                    value: val,
                    wrap: self.wrap + rhs.wrap,
                },
            }
        }
    }
}

impl std::ops::Sub for Mod100 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.value == 0 {
            Mod100 {
                value: rhs.value.rem_euclid(100),
                wrap: self.wrap + rhs.wrap,
            }
        } else {
            let val = self.value - rhs.value;
            match val {
                v if !(1..100).contains(&v) => Mod100 {
                    value: v.rem_euclid(100),
                    wrap: self.wrap + rhs.wrap + 1,
                },
                _ => Mod100 {
                    value: val,
                    wrap: self.wrap + rhs.wrap,
                },
            }
        }
    }
}

impl Deref for Mod100 {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FromStr for Mod100 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: i32 = s.parse().map_err(|_| Error::ParseNumber)?;
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
            count: count.parse().map_err(|_| Error::ParseNumber)?,
        })
    }
}

fn main() {
    println!(
        "{:?}",
        stdin()
            .lock()
            .lines()
            .map(|v| {
                v.map_err(|_| Error::Io)
                    .and_then(|v| v.parse::<Row>())
                    .map(|v| match v {
                        Row {
                            direction: Direction::Left,
                            count,
                        } => Mod100 {
                            value: -count.value,
                            wrap: count.wrap,
                        },
                        Row {
                            direction: Direction::Right,
                            count,
                        } => count,
                    })
            })
            .fold(Mod100::from(50), |cursor, op| {
                if let Ok(movement) = op {
                    cursor + movement
                } else {
                    cursor
                }
            })
    );
}
