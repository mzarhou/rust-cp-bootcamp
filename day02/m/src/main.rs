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
    let (m, n) = next_tuple::<usize>();

    let s = sieve(n);
    let is_absent = {
        let mut c = 0;
        for i in m..=n {
            if s[i] {
                c += 1;
                println!("{}", i);
            }
        }
        c == 0
    };
    if is_absent {
        println!("Absent");
    }
}
