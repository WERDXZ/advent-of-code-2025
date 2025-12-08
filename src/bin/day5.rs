#![feature(range_into_bounds)]
#![feature(range_bounds_is_empty)]
use std::io::{BufRead, stdin};
use std::ops::{IntoBounds, RangeBounds, RangeInclusive};

fn main() {
    let ranges: Vec<RangeInclusive<u64>> = stdin()
        .lock()
        .lines()
        .map_while(|line| {
            line.ok().and_then(|line| {
                line.split_once('-').and_then(|(start, end)| {
                    start
                        .parse::<u64>()
                        .ok()
                        .and_then(|start| end.parse::<u64>().ok().map(|end| start..=end))
                })
            })
        })
        .fold(Vec::<RangeInclusive<u64>>::new(), |mut vec, range| {
            let index = vec.iter().enumerate().find_map(|(index, value)| {
                if value.clone().intersect(range.clone()).is_empty() {
                    None
                } else {
                    Some(index)
                }
            });
            if let Some(index) = index {
                let existing = vec[index].clone();
                let new_range =
                    *existing.start().min(range.start())..=*existing.end().max(range.end());
                vec.remove(index);
                vec.push(new_range);
                vec
            } else {
                vec.push(range);
                vec
            }
        });

    let queries: u64 = stdin()
        .lock()
        .lines()
        .map_while(|line| line.ok().and_then(|line| line.parse::<u64>().ok()))
        .map(|v| {
            if ranges.iter().any(|range| range.contains(&v)) {
                1
            } else {
                0
            }
        })
        .sum();

    println!("{}", queries);
}
