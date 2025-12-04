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
            .flat_map(patterns)
            .flat_map(move |(range, len)| {
                (nearest_next_multiple(lhs, range)
                    .max(nearest_next_multiple(10_i64.pow(len - 1), range))
                    ..=rhs.min(10_i64.pow(len) - 1))
                    .step_by(range as usize)
            })
    }
}

fn nearest_next_multiple(value: i64, multiple: i64) -> i64 {
    match value % multiple {
        0 => value,
        _ => value + (multiple - (value % multiple)),
    }
}

fn patterns(len: usize) -> impl Iterator<Item = (i64, u32)> {
    (1..len).filter(move |v| len.is_multiple_of(*v)).map(move |factor| {
        (
            (0..len / factor)
                .map(|count| 10_i64.pow(count as u32 * factor as u32))
                .sum::<i64>(),
            len as u32,
        )
    })
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
            .collect::<std::collections::HashSet<_>>()
            .iter()
            .sum::<i64>()
    );
}

#[cfg(test)]
mod test{
    use super::patterns;
    #[test]
    fn test_patterns(){
        let result:Vec<(i64,u32)>=patterns(4).collect();
        assert_eq!(result,vec![(1111,4),(101,4)]);
        let result:Vec<(i64,u32)>=patterns(5).collect();
        assert_eq!(result,vec![(11111, 5)]);
        let result:Vec<(i64,u32)>=patterns(8).collect();
        assert_eq!(result,vec![(11111111, 8), (1010101, 8), (10001, 8)]);
        let result:Vec<(i64,u32)>=patterns(9).collect();
        assert_eq!(result,vec![(111111111, 9), (1001001, 9)]);
    }
}
