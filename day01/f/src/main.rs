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

fn solve(
    x: usize,
    coins: &[usize],
    ready: &mut [bool],
    value: &mut [Option<usize>],
) -> Option<usize> {
    if x == 0 {
        return Some(0);
    }
    if ready[x] {
        return value[x];
    }
    let m = coins
        .iter()
        .filter_map(|c| {
            if *c > x {
                return None;
            }
            solve(x - c, coins, ready, value)
        })
        .min();
    ready[x] = true;
    if let Some(minimum) = m {
        value[x] = Some(minimum + 1);
        return Some(minimum + 1);
    }
    value[x] = None;
    return None;
}

fn main() {
    let (_n, x) = next_tuple::<usize>();
    let coins = next_line::<usize>();
    let mut ready = vec![false; x + 1];
    let mut value = vec![None; x + 1];
    if let Some(min_coins) = solve(x, coins.as_slice(), &mut ready, &mut value) {
        println!("{min_coins}");
    } else {
        println!("-1");
    }
}

// fn solve(x: u64, coins: &[u64], cach: &mut HashMap<u64, Option<u64>>) -> Option<u64> {
//     if x == 0 {
//         return Some(0);
//     }
//     if let Some(value) = cach.get(&x) {
//         return *value;
//     }
//     let m = coins
//         .iter()
//         .filter_map(|c| {
//             if *c > x {
//                 return None;
//             }
//             let value = solve(x - c, coins, cach);
//             cach.insert(x - c, value);
//             return value;
//         })
//         .min();
//     if let Some(minimum) = m {
//         return Some(1 + minimum);
//     }
//     return None;
// }

// fn solve(x: usize, coins: &[usize], cach: &mut HashMap<usize, Option<usize>>) -> Option<usize> {
//     if x == 0 {
//         return Some(0);
//     }
//     if let Some(value) = cach.get(&x) {
//         return *value;
//     }
//     let m = coins
//         .iter()
//         .filter_map(|c| {
//             if *c > x {
//                 return None;
//             }
//             solve(x - c, coins, cach)
//         })
//         .min();
//     if let Some(minimum) = m {
//         cach.insert(x, Some(minimum + 1));
//         return Some(minimum + 1);
//     }
//     cach.insert(x, None);
//     return None;
// }

// fn main() {
//     let (_n, x) = next_tuple::<usize>();
//     let coins = next_line::<usize>();
//     let mut cach = HashMap::new();
//     if let Some(min_coins) = solve(x, coins.as_slice(), &mut cach) {
//         println!("{min_coins}");
//     } else {
//         println!("-1");
//     }
// }
