use std::{
    collections::VecDeque,
    io::{BufRead, stdin},
};

fn main() {
    let input: Vec<String> = stdin().lock().lines().map_while(Result::ok).collect();

    let line1 = input[0].chars().collect::<Vec<char>>();
    let line2 = input[1].chars().collect::<Vec<char>>();
    let line3 = input[2].chars().collect::<Vec<char>>();
    let line4 = input[3].chars().collect::<Vec<char>>();
    let line5 = input[4].chars().collect::<Vec<char>>();

    let res = line1
        .iter()
        .zip(line2.iter())
        .zip(line3.iter())
        .zip(line4.iter())
        .zip(line5.iter())
        .rev()
        .fold(
            (0i64, VecDeque::<i64>::new()),
            |(sum, mut stack), ((((a, b), c), d), op)| {
                let number = match String::from_iter([*a, *b, *c, *d]).trim().parse::<i64>() {
                    Ok(v) => v,
                    Err(_) => return (sum, stack),
                };
                stack.push_back(number);
                match op {
                    '+' => (sum + stack.into_iter().sum::<i64>(), VecDeque::new()),
                    '*' => (sum + stack.into_iter().product::<i64>(), VecDeque::new()),
                    _ => (sum, stack),
                }
            },
        )
        .0;

    println!("{}", res);
}
