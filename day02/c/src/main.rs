fn solve(a: u64, b: u64) -> u64 {
    let m = (1e9 as u64) + 7;
    if b == 0 {
        return 1;
    }
    let result = solve(a, b / 2);
    if b % 2 != 0 {
        return (((result * result) % m) * (a % m)) % m;
    }
    return (result * result) % m;
}

fn main() {
    let n = {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer.trim().parse::<u64>().unwrap()
    };
    println!("{}", solve(3, n));
}
