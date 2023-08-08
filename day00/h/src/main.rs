use std::collections::BinaryHeap;
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

fn main() {
    let (n, q) = next_tuple::<usize>();
    let mut bh_queues = vec![BinaryHeap::<i64>::new(); n];

    for _ in 0..q {
        let line = next_line::<i64>();
        match line[0] {
            0 => {
                let queue = bh_queues.get_mut(line[1] as usize).unwrap();
                queue.push(line[2]);
            }
            1 => {
                let queue = bh_queues.get_mut(line[1] as usize).unwrap();
                if let Some(max) = queue.peek() {
                    println!("{}", max);
                }
            }
            2 => {
                let queue = bh_queues.get_mut(line[1] as usize).unwrap();
                queue.pop();
            }
            _ => {}
        }
    }
}
