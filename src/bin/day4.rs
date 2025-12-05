use std::{
    collections::VecDeque,
    io::{BufRead, stdin},
};

fn main() {
    let input = stdin()
        .lock()
        .lines()
        .flat_map(|v| {
            v.ok().map(|v| {
                v.chars()
                    .map(|c| match c {
                        '.' => 0,
                        '@' => 1,
                        _ => 0,
                    })
                    .collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>();
    let mut position = input.clone();
    let mut neighbor = input.clone();
    let (width, height) = (input[0].len(), input.len());
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    println!(
        "{:?}",
        std::iter::repeat(())
            .scan(
                (
                    (0..width)
                        .flat_map(|v| std::iter::repeat(v).zip(0..height))
                        .filter_map(|(col, row)| if input[row][col] == 0 {
                            None
                        } else {
                            neighbor[row][col] = offsets
                                .iter()
                                .filter_map(|(offset_w, offset_h)| {
                                    input
                                        .get(row.overflowing_add_signed(*offset_h).0)
                                        .and_then(|row| {
                                            row.get(col.overflowing_add_signed(*offset_w).0)
                                        })
                                        .copied()
                                })
                                .sum::<i64>();
                            if neighbor[row][col] < 4 {
                                position[row][col] = 0;
                                Some((row, col))
                            } else {
                                None
                            }
                        })
                        .collect::<VecDeque<_>>(),
                    neighbor
                ),
                |(queue, neighbor), _| queue
                    .pop_front()
                    .map(|(row, col)| offsets
                        .iter()
                        .map(move |(offset_w, offset_h)| (
                            row.overflowing_add_signed(*offset_h).0,
                            col.overflowing_add_signed(*offset_w).0
                        ))
                        .filter_map(|(row_o, col_o)| {
                            neighbor
                                .get_mut(row_o)
                                .and_then(|row| row.get_mut(col_o))
                                .map(|v| {
                                    *v -= 1;
                                    *v
                                })
                                .map(|v| (row_o, col_o, v.max(0)))
                        })
                        .filter(|(_, _, v)| *v < 4)
                        .filter(|(row, col, _)| {
                            position
                                .get_mut(*row)
                                .and_then(|row| row.get_mut(*col))
                                .map(|v| match *v {
                                    0 => false,
                                    _ => {
                                        *v = 0;
                                        true
                                    }
                                })
                                .unwrap_or(false)
                        }))
                    .map(|v| v.fold((), |_, (row, col, _)| queue.push_back((row, col))))
            )
            .count()
    );
}
