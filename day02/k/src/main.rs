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

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

fn main() {
    let n = next_line::<usize>()[0];
    let numbers = next_line::<u64>();

    let mut prefix = vec![0; n];
    prefix[0] = 0;
    for i in 1..=n - 1 {
        prefix[i] = gcd(prefix[i - 1], numbers[i - 1]);
    }

    let mut suffix = vec![0; n];
    suffix[n - 1] = 0;
    for i in (0..=n - 2).rev() {
        suffix[i] = gcd(suffix[i + 1], numbers[i + 1]);
    }

    let mut best_max = 0;
    for i in 0..n {
        let result = gcd(prefix[i], suffix[i]);
        if result > best_max {
            best_max = result;
        }
    }

    println!("{}", best_max);
}
