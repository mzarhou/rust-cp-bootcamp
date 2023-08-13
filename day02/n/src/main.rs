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

fn sieve(n: usize) -> Vec<bool> {
    let mut v = vec![true; n + 1];
    v[0] = false;
    v[1] = false;

    for i in 2..=n {
        if !v[i] {
            continue;
        }
        for j in (i * i..=n).step_by(i) {
            v[j] = false;
        }
    }
    return v;
}

fn main() {
    let _n = next_line::<u64>()[0];
    let numbers = next_line::<u64>();

    let sv = sieve((*numbers.iter().max().unwrap() as f64).sqrt() as usize);

    for n in numbers {
        let x = (n as f64).sqrt() as u64;
        if x * x != n {
            println!("NO");
        } else {
            println!("{}", if sv[x as usize] { "YES" } else { "NO" })
        }
    }
}
