use std::io;

fn main() -> io::Result<()> {
    let mut stack: Vec<String> = Vec::with_capacity(1_000_000);
    let mut buffer = String::with_capacity(10);
    let stdin = io::stdin();
    let mut index = 0;

    io::stdin().read_line(&mut buffer)?;
    let t = buffer.trim().parse::<u32>().unwrap();

    let mut output: Vec<String> = Vec::with_capacity(1_000_000);

    buffer.clear();

    while let Some(_) = stdin.read_line(&mut buffer).ok() {
        index += 1;
        let line = buffer.trim();
        match line {
            "2" => {
                stack.pop();
            }
            "3" => {
                if stack.len() == 0 {
                    output.push("Empty!\n".to_string());
                } else {
                    output.push(format!("{}\n", stack.last().unwrap()));
                }
            }
            _ => {
                let (_, value_str) = line.split_once(" ").unwrap();
                stack.push(value_str.to_string());
            }
        }
        buffer.clear();
        if index == t {
            break;
        }
    }

    print!("{}", output.join(""));
    Ok(())
}
