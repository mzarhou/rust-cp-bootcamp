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

fn main() {
    let (n, q) = next_tuple::<usize>();
    let values = next_line::<i64>();
    let mut pre = vec![0; n + 1];
    for index in 1..=n {
        pre[index] = pre[index - 1] + values[index - 1];
    }
    for _ in 0..q {
        let (from, to) = next_tuple::<usize>();
        println!("{}", pre[to] - pre[from - 1]);
    }
}
