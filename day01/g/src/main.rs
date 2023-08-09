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

fn main() {
    let _n = next_line::<usize>()[0];
    let numbers = next_line::<i64>();

    let mut prev = numbers[0];
    let mut counter = 0;
    for (idx, current) in numbers.iter().skip(1).enumerate() {
        if prev < numbers[idx] {
            prev = numbers[idx];
        }
        let diff: i64 = prev - current;
        if diff > 0 {
            counter += diff;
        }
    }
    println!("{counter}");
}
