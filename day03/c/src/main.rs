use std::collections::HashMap;
use std::io::stdin;

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

fn factoriel(n: u64, m: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    return ((n % m) * factoriel(n - 1, m)) % m;
}

fn expo(a: u64, b: u64, m: u64) -> u64 {
    if b == 0 {
        return 1;
    }
    let result = expo(a, b / 2, m);
    if b % 2 != 0 {
        return (((result * result) % m) * (a % m)) % m;
    }
    return (result * result) % m;
}

fn main() {
    let m = 1e9 as u64 + 7;
    let s = next_line::<String>()[0].to_string();
    let n = s.len();

    let mut map = HashMap::<char, u64>::new();
    for c in s.chars() {
        *map.entry(c).or_default() += 1;
    }
    let occur = map.values().fold(1 as u64, |result, value| {
        (result * factoriel(*value, m)) % m
    });
    println!("{}", (factoriel(n as u64, m) * expo(occur, m - 2, m)) % m);
}
