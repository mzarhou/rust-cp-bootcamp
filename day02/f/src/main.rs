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

// check we can make nb_products in `time` time
fn f(time: i64, t_machines: &[i64], nb_products: i64) -> bool {
    t_machines.iter().map(|t_i| time / t_i).sum::<i64>() >= nb_products
}

fn main() {
    let (_n, nb_products) = next_tuple::<i64>();
    let t_machines = next_line::<i64>();

    let mut best_min_time = i64::MAX;
    let mut lo = 0 as i64;
    let mut hi = {
        let mut i = 1;
        loop {
            if f(i, &t_machines, nb_products) {
                break i;
            }
            i *= 2;
        }
    };

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if f(mid, &t_machines, nb_products) {
            hi = mid - 1;
            best_min_time = std::cmp::min(best_min_time, mid);
        } else {
            lo = mid + 1;
        }
    }
    println!("{best_min_time}")
}
