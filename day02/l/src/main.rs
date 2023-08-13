fn prime_fact(n: usize) -> Vec<usize> {
    let mut n = n;
    let mut v = vec![];
    while n % 2 == 0 {
        v.push(2);
        n /= 2;
    }
    for i in (3..=(n as f64).sqrt() as usize).step_by(2) {
        while n % i == 0 {
            v.push(i);
            n /= i;
        }
    }

    if n > 2 {
        print!(" {}", n);
        v.push(n);
    }
    return v;
}

fn main() {
    let mut n = {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer.trim().parse::<usize>().unwrap()
    };

    print!("{}:", n);
    while n % 2 == 0 {
        print!(" 2");
        n /= 2;
    }
    for i in (3..=(n as f64).sqrt() as usize).step_by(2) {
        while n % i == 0 {
            print!(" {}", i);
            n /= i;
        }
    }

    if n > 2 {
        print!(" {}", n);
    }
    println!("");
}
