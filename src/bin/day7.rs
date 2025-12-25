use std::io::{BufRead, stdin};

fn main() {
    let lines: Vec<_> = stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .step_by(2)
        .collect();
    let width = lines[0].len();
    let start = lines[0].find('S').unwrap();

    let result: u64 = lines[1..]
        .iter()
        .map(|s| s.chars().map(|c| c == '^').collect::<Vec<_>>())
        .fold(
            {
                let mut v = vec![0u64; width];
                v[start] = 1;
                v
            },
            |counts, splitters| {
                (0..width).fold(vec![0u64; width], |mut new, col| {
                    if splitters[col] {
                        if col > 0 {
                            new[col - 1] += counts[col];
                        }
                        if col + 1 < width {
                            new[col + 1] += counts[col];
                        }
                    } else {
                        new[col] += counts[col];
                    }
                    new
                })
            },
        )
        .iter()
        .sum();

    println!("{result}");
}
