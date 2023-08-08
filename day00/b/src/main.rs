use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let n = buffer.trim().parse::<u8>().unwrap();
    if n < 1 || n > 100 {
        panic!()
    }
    buffer.clear();
    io::stdin().read_line(&mut buffer)?;
    let mut vec = buffer
        .trim()
        .split(" ")
        .map(|i| {
            let value = i.parse::<u8>().unwrap();
            if value < 1 || value > 100 {
                panic!()
            }
            return value;
        })
        .collect::<Vec<u8>>();
    if vec.len() != n.into() {
        panic!()
    }
    vec.sort();
    let result = vec
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{result}");
    Ok(())
}
