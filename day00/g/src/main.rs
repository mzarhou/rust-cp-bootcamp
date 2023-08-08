use std::{collections::HashMap, io::stdin};
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

fn find_positions(array: &[i64], sum: i64) -> Option<(usize, usize)> {
    let mut map = HashMap::new();
    for (pos, value) in array.into_iter().enumerate() {
        let remaining: i64 = sum - value;
        if remaining < 0 {
            map.insert(value, pos);
            continue;
        }
        if let Some(l_pos) = map.get(&remaining) {
            return Some(((*l_pos + 1), pos + 1));
        } else {
            map.insert(value, pos);
        }
    }
    return None;
}

fn main() {
    let (_n, x) = next_tuple::<i64>();
    let numbers = next_line::<i64>();
    if let Some((i, j)) = find_positions(numbers.as_slice(), x) {
        println!("{i} {j}");
    } else {
        println!("IMPOSSIBLE");
    }
}
