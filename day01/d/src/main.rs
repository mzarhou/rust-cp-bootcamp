use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();

    let mut v: Vec<u64> = vec![0; 45];
    v[0] = 1;
    v[1] = 1;
    for i in 2..=n {
        v[i] = v[i - 1] + v[i - 2];
    }
    println!("{}", v[n]);
}
