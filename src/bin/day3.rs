use std::{
    io::{BufRead, stdin},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq)]
struct Bank {
    batteries: Vec<char>,
}

impl FromStr for Bank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Bank {
            batteries: s.chars().collect(),
        })
    }
}

impl Bank {
    fn solve<const SIZE: usize>(self) -> i64 {
        self.batteries
            .iter()
            .map(|c| c.to_digit(10).unwrap_or(0))
            .fold(
                (Vec::<u32>::new(), self.batteries.len() - SIZE),
                |(mut stack, mut quota), v| {
                    while let Some(last) = stack.last()
                        && *last < v
                        && quota > 0
                    {
                        stack.pop();
                        quota -= 1;
                    }
                    stack.push(v);
                    (stack, quota)
                },
            )
            .0
            .iter()
            .take(SIZE)
            .rev()
            .enumerate()
            .map(|(index, value)| 10i64.pow(index as u32) * *value as i64)
            .sum::<i64>()
    }
}

fn main() {
    println!(
        "{}",
        stdin()
            .lock()
            .lines()
            .flat_map(|v| v.ok().and_then(|line| line.parse::<Bank>().ok()))
            .map(Bank::solve::<12>)
            .sum::<i64>()
    );
}
