use std::{collections::HashSet, io};
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut numbers = HashSet::new();
    while let Ok(nb_bytes) = stdin.read_line(&mut buffer) {
        if nb_bytes == 0 {
            break;
        }
        let (action, n) = buffer.trim().split_once(" ").unwrap();
        let number = n.parse::<i32>().unwrap();
        match action {
            "insert" => {
                numbers.insert(number);
            }
            "delete" => {
                numbers.remove(&number);
            }
            "exists" => {
                if numbers.contains(&number) {
                    println!("true");
                } else {
                    println!("false");
                }
            }
            _ => {}
        }
        buffer.clear();
    }
    Ok(())
}
