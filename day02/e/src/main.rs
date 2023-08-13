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
    let n = next_line::<usize>()[0];
    for _ in 0..n {
        let (a, b, c) = {
            let v = next_line::<f64>();
            (v[0], v[1], v[2])
        };

        if c == 0.0 {
            println!("{}", c);
            return;
        }
        let press = 1e-6;
        let mut lo = 0 as f64;
        let mut hi = f64::MAX;

        let mut mid = 0.0;
        while (hi - lo) > press {
            mid = lo + (hi - lo) / 2 as f64;
            if mid > c {
                hi = mid;
                continue;
            }
            let result = a * mid + b * mid.sin();
            if result > c {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        println!("{:.6}", mid);
    }
}
