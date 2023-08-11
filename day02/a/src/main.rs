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

fn find(collection: &[i64], target: i64, n: usize) -> bool {
    let mut l = 0;
    let mut r = n - 1;
    let mut m;

    while l <= r {
        m = (l + r) / 2;
        if collection[m] == target {
            return true;
        }
        if collection[m] < target {
            l = m + 1;
        } else if m == 0 {
            break;
        } else {
            r = m - 1;
        }
    }
    return false;
}

fn main() {
    let n = next_line::<usize>()[0];
    let collection = next_line::<i64>();
    let _m = next_line::<usize>()[0];
    let targets = next_line::<i64>();

    for target in targets {
        if find(collection.as_slice(), target, n) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
