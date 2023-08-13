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

fn pow(a: u64, b: u64, m: u64) -> u64 {
    if b == 0 {
        return 1;
    }
    let result = pow(a, b / 2, m);
    if b % 2 != 0 {
        return (((result * result) % m) * (a % m)) % m;
    }
    return (result * result) % m;
}

fn main() {
    let m = 1000000007;

    let (n, q) = {
        let v = next_line::<usize>();
        (v[0], v[1])
    };

    let numbers = next_line::<u64>();
    let mut pre = vec![1 as u64; n + 1];
    for i in 1..=n {
        pre[i] = ((pre[i - 1] % m) * (numbers[i - 1] % m)) % m;
    }

    for _ in 0..q {
        let (from, to) = {
            let v = next_line::<usize>();
            (v[0], v[1])
        };
        // if m is prime -> a^(m - 1) = 1 mod m
        let result = (pre[to] * pow(pre[from - 1], m - 2, m)) % m;
        println!("{}", result);
    }
}
