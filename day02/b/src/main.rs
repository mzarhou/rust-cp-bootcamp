use std::io::stdin;

fn next_tuple<T: std::str::FromStr>() -> (T, T)
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    T: Clone,
{
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("next_tuple: read");
    let v: Vec<T> = buffer
        .split_whitespace()
        .take(2)
        .map(|a| a.parse().expect("next_tuple: parse"))
        .collect();
    (v[0].clone(), v[1].clone())
}

fn solve(a: u64, b: u64) -> u64 {
    let m = (1e9 as u64) + 7;
    if b == 0 {
        return 1;
    }
    let result = solve(a, b / 2);
    if b % 2 != 0 {
        return (((result * result) % m) * (a % m)) % m;
    }
    return (result * result) % m;
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    for _ in 0..n {
        let (a, b) = next_tuple::<u64>();
        println!("{}", solve(a, b));
    }
}
