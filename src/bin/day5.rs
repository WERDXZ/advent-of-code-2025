use std::io::{BufRead, stdin};
use std::ops::RangeInclusive;

fn main() {
    let mut ranges: Vec<RangeInclusive<u64>> = stdin()
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
        .collect();

    ranges.sort_by_key(|r| *r.start());

    let ranges =
        ranges
            .into_iter()
            .fold(Vec::new(), |mut merged: Vec<RangeInclusive<u64>>, range| {
                if let Some(last) = merged.last_mut() {
                    // Check if overlapping or adjacent (end + 1 >= start)
                    if *last.end() + 1 >= *range.start() {
                        // Extend end if needed
                        if *range.end() > *last.end() {
                            *last = *last.start()..=*range.end();
                        }
                    } else {
                        merged.push(range);
                    }
                } else {
                    merged.push(range);
                }
                merged
            });

    println!("{}", ranges.into_iter().map(|v| v.count()).sum::<usize>());
}
