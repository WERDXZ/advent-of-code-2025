#![feature(array_windows)]
use std::{
    hash::Hash,
    io::{BufRead, stdin},
    ops::Add,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    pub x: i64,
    pub y: i64,
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let mut input: Vec<Point> = stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .map_while(|s| {
            s.split_once(',')
                .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        })
        .map(Into::into)
        .collect();

    input.push(input[0]);

    let (horizontal, vertical): (Vec<[Point; 2]>, Vec<[Point; 2]>) =
        input.array_windows().partition(|[a, b]| a.y == b.y);

    let res = (0..input.len() - 1)
        .map(|i| {
            (i + 1..input.len() - 1)
                .filter_map(|j| {
                    let p1 = input[i];
                    let p2 = input[j];
                    let p3 = Point { x: p2.x, y: p1.y };
                    let p4 = Point { x: p1.x, y: p2.y };
                    let edge1 = [p1, p3]; // horizontal
                    let edge2 = [p4, p2]; // horizontal
                    let edge3 = [p1, p4]; // vertical
                    let edge4 = [p3, p2]; // vertical

                    if !inside_polygon(&p3, &vertical) {
                        return None;
                    }

                    if !inside_polygon(&p4, &vertical) {
                        return None;
                    }

                    for edge in horizontal.iter() {
                        // check intersection with vertical edges
                        if intersect(edge, &edge3) || intersect(edge, &edge4) {
                            return None;
                        }
                    }

                    for edge in vertical.iter() {
                        // check intersection with horizontal edges
                        if intersect(&edge1, edge) || intersect(&edge2, edge) {
                            return None;
                        }
                    }

                    Some(((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1))
                })
                .max()
        })
        .max();

    println!("{:?}", res);
}

fn intersect(horizontal: &[Point; 2], vertical: &[Point; 2]) -> bool {
    let hx_min = horizontal[0].x.min(horizontal[1].x);
    let hx_max = horizontal[0].x.max(horizontal[1].x);

    let vy_min = vertical[0].y.min(vertical[1].y);
    let vy_max = vertical[0].y.max(vertical[1].y);

    let x = vertical[0].x; // vertical line x
    let y = horizontal[0].y; // horizontal line y

    x > hx_min && x < hx_max && y > vy_min && y < vy_max
}

fn inside_polygon(point: &Point, verticals: &Vec<[Point; 2]>) -> bool {
    let mut count = 0;
    for edge in verticals {
        let edge_x = edge[0].x;
        let (min_y, max_y) = if edge[0].y < edge[1].y {
            (edge[0].y, edge[1].y)
        } else {
            (edge[1].y, edge[0].y)
        };

        if edge_x > point.x && point.y > min_y && point.y < max_y {
            count += 1;
        }
    }
    count % 2 == 1
}
