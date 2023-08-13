use std::{collections::HashMap, io::stdin};

fn next_line<T: std::str::FromStr>() -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("next_line: read");
    buffer
        .split_whitespace()
        .map(|a| a.parse().expect("next_line: parse"))
        .collect()
}

fn main() {
    let n = next_line::<u64>()[0];
    let numbers = next_line::<u64>();

    let mut best_max = 1;

    let mut lo = 1;
}
