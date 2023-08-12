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

fn good(max: i64, numbers: &[i64], k: i64) -> bool {
    let mut count = 0;
    let mut sum = 0;
    for n in numbers {
        sum += n;
        if sum > max {
            count += 1;
            sum = *n;
        }
    }
    if sum != 0 {
        count += 1;
    }
    return count <= k;
}

fn main() {
    let (_n, k) = next_tuple::<i64>();
    let numbers = next_line::<i64>();

    let mut lo = *numbers.iter().max().unwrap();
    let mut hi = numbers.iter().sum::<i64>();
    let mut best_value = i64::MAX;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if good(mid, &numbers, k) {
            best_value = std::cmp::min(best_value, mid);
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }

    println!("{best_value}");
}
