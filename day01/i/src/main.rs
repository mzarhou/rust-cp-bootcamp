use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    let _ = stdin().read_line(&mut buffer);
    let n = buffer.trim().parse::<i64>().unwrap();
    let sum: i64 = (1..=n).sum();

    if sum % 2 == 1 {
        return println!("NO");
    }

    let mut v1 = vec![];
    let mut v2 = vec![];
    let mut half = sum / 2;
    for value in (1..=n).into_iter().rev() {
        if half - value >= 0 {
            v1.push(value);
            half -= value;
        } else {
            v2.push(value);
        }
    }
    let v1: Vec<String> = v1.into_iter().map(|i| i.to_string()).collect();
    let v2: Vec<String> = v2.into_iter().map(|i| i.to_string()).collect();
    println!(
        "YES {} {} {} {}",
        v1.len(),
        v1.join(" "),
        v2.len(),
        v2.join(" ")
    )
}
