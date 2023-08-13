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

fn main() {
    let numbers = next_line::<i64>();
    let m = 998244353 as i64;
    let a = numbers[0] % m;
    let b = numbers[1] % m;
    let c = numbers[2] % m;
    let d = numbers[3] % m;
    let e = numbers[4] % m;
    let f = numbers[5] % m;

    let mut result = ((((a * b) % m) * c) % m - (((d * e) % m) * f) % m) % m;
    if result < 0 {
        result += m;
    }
    println!("{}", result);
}
