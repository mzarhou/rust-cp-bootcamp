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

fn main() {
    let (n, x) = next_tuple::<i64>();
    let n = n as usize;
    let numbers = next_line::<i64>();

    let mut map = HashMap::<i64, usize>::new();
    let mut curr_sum = 0;
    let mut counter = 0;
    for idx in 0..n {
        curr_sum += numbers[idx];
        if curr_sum == x {
            counter += 1;
        }
        if let Some(val) = map.get(&(curr_sum - x)) {
            counter += val;
        }
        *map.entry(curr_sum).or_default() += 1;
    }
    println!("{counter}")
}

// v2
// fn main() {
//     let (n, x) = next_tuple::<i64>();
//     let n = n as usize;
//     let numbers = next_line::<i64>();

//     let mut pre = vec![0; n + 1];
//     let mut map = HashMap::<i64, usize>::new();
//     let mut curr_sum;
//     let mut counter = 0;
//     for idx in 1..=n {
//         pre[idx] = pre[idx - 1] + numbers[idx - 1];
//         curr_sum = pre[idx];
//         if curr_sum == x {
//             counter += 1;
//         }
//         if let Some(val) = map.get(&(curr_sum - x)) {
//             counter += val;
//         }
//         *map.entry(curr_sum).or_default() += 1;
//     }
//     println!("{counter}")
// }

//
// fn main() {
//     let (n, x) = next_tuple::<i64>();
//     let n = n as usize;
//     let numbers = next_line::<i64>();

//     let mut pre = vec![0; n + 1];
//     let mut counter = 0;
//     for idx in 1..=n {
//         pre[idx] = pre[idx - 1] + numbers[idx - 1];
//     }

//     for i in 1..=n {
//         for j in i..=n {
//             let result = pre[j] - pre[i - 1];
//             if result == x {
//                 counter += 1;
//             }
//         }
//     }
//     println!("{counter}")
// }
