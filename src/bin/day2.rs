use std::{
    io::{stdin, BufRead},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Range {
    left: String,
    right: String,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let left = parts.next().ok_or(())?.to_string();
        let right = parts.next().ok_or(())?.to_string();
        Ok(Range { left, right })
    }
}

impl Range {
    fn resolve(self) -> impl Iterator<Item = i64> {
        let lhs = self.left.parse::<i64>().unwrap();
        let rhs = self.right.parse::<i64>().unwrap();
        (self.left.len()..=self.right.len())
            .filter_map(|v| match v % 2 {
                1 => None,
                0 => Some(10_i64.pow(v as u32 / 2) + 1),
                _ => unreachable!(),
            })
            .flat_map(move |r| {
                (match lhs % r {
                    0 => lhs,
                    _ => lhs + (r - (lhs % r)),
                }
                .max(r * (r / 10))..=rhs.min(r * (r - 2)))
                    .step_by(r as usize)
            })
    }
}

fn main() {
    println!(
        "{}",
        stdin()
            .lock()
            .lines()
            .flat_map(|v| v
                .map(|v| v.split(',').map(String::from).collect::<Vec<_>>())
                .ok())
            .flatten()
            .flat_map(|v| v.parse::<Range>().ok())
            .flat_map(Range::resolve)
            .sum::<i64>()
    );
}
