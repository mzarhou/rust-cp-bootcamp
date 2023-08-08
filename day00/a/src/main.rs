use std::io;
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let Ok(n) = buffer.trim().parse::<u8>() else {
        panic!()
    };
    if n > 100 {
        panic!()
    }

    buffer.clear();
    io::stdin().read_line(&mut buffer)?;

    let result = buffer
        .trim()
        .split(" ")
        .map(|i| {
            let Ok(value) =  i.parse::<i16>() else {
                panic!()
            };
            if value.abs() > 100 {
                panic!()
            }
            if value < 0 {
                return value.to_string();
            }
            return (value + 2).to_string();
        })
        .collect::<Vec<String>>()
        .join(" ");

    println!("{result}");
    Ok(())
}
