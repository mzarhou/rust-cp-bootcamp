fn good(k: i64, n: i64) -> bool {
    // equation k * (k + 1)
    k <= (n * 2) / (k + 1)
}

fn main() {
    let n = {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer.trim().parse::<i64>().unwrap()
    };

    let mut best_len = 0;
    let mut lo = 0 as i64;
    let mut hi = {
        let mut i = 1;
        while good(i, n) {
            i *= 2;
        }
        i * (i + 1) / 2
    };

    while lo <= hi {
        let stairs_len = lo + (hi - lo) / 2;
        if good(stairs_len, n) {
            best_len = stairs_len;
            lo = stairs_len + 1;
        } else {
            hi = stairs_len - 1;
        }
    }
    println!("{}", best_len);
}
