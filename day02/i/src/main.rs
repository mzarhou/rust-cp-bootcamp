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

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

fn main() {
    let n = next_line::<usize>()[0];
    let numbers = next_line::<i64>();
    let mut result = None;

    for i in 1..n {
        let a = numbers[i - 1];
        let b = numbers[i];
        if let Some(res) = result {
            result = Some(std::cmp::min(res, gcd(a, b)));
        } else {
            result = Some(gcd(a, b));
        }
    }

    if let Some(result) = result {
        println!("{}", result);
    } else if n == 1 {
        println!("{}", numbers[0]);
    } else {
        println!("{}", i64::MAX);
    }
}
