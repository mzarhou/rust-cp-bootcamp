use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<usize>().unwrap();
    let mut chars = vec![];
    for _ in 0..n {
        line.clear();
        stdin().read_line(&mut line).unwrap();
        let chs = line
            .trim()
            .split("")
            .filter_map(|s| s.chars().next())
            .collect::<Vec<_>>();
        chars.push(chs);
    }

    if chars[0][0] == '*' || chars[n - 1][n - 1] == '*' {
        println!("0");
        return;
    }

    let mut v = vec![vec![0; n + 1]; n + 1];
    let md = 1e9 as u64 + 7;
    v[0][1] = 1;
    for i in 1..=n {
        for j in 1..=n {
            if chars[i - 1][j - 1] == '*' {
                v[i][j] = 0;
            } else {
                v[i][j] = (v[i - 1][j] + v[i][j - 1]) % md;
            }
        }
    }
    println!("{}", v[n][n])
}
