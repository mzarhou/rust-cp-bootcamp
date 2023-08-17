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
    let n = next_line::<usize>()[0];
    for _ in 0..n {
        let (a, b, c) = {
            let v = next_line::<u64>();
            (v[0], v[1], v[2])
        };
        let result = expo(a, expo(b, c, m - 1), m);
        println!("{}", result);
    }
}
