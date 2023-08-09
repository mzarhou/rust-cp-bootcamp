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

fn next_str() -> String {
    let mut buffer = String::new();
    let _ = stdin().read_line(&mut buffer);
    buffer = buffer.trim().to_string();
    return buffer;
}

fn main() {
    let t = next_str().parse::<usize>().unwrap();

    for _ in 0..t {
        let (n, d) = next_tuple::<String>();
        let mut value = next_str();

        let d = d.chars().next().unwrap();
        let n = n.parse::<usize>().unwrap();
        for (index, c) in value.clone().chars().enumerate() {
            if c < d {
                value.insert(index, d);
                break;
            }
        }
        if value.len() == n {
            value.insert(n, d);
        }
        println!("{value}");
    }
}
