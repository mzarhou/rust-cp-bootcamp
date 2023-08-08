use std::io;

fn check_str(line: &mut String) -> bool {
    let mut counter = 0;
    if line.len() == 0 {
        return true;
    }
    if line.len() % 2 == 1 {
        return false;
    }
    let opening_char = line.bytes().next().unwrap();
    match opening_char {
        b'[' | b'(' => {
            let closing_char = if opening_char == b'[' { b']' } else { b')' };
            let mut index = 1;
            while index < line.len() {
                let c = line.bytes().nth(index).unwrap();
                if c == opening_char {
                    counter += 1;
                }
                if c != closing_char {
                    index += 1;
                    continue;
                }
                if counter > 0 {
                    counter -= 1;
                    index += 1;
                    continue;
                }
                line.remove(index);
                line.remove(0);
                return check_str(line);
            }
            return false;
        }
        _ => {
            return false;
        }
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    let n = buffer.trim().parse::<u8>().unwrap();
    buffer.clear();
    let mut index = 0;
    while let Some(_) = stdin.read_line(&mut buffer).ok() {
        index += 1;
        if check_str(&mut buffer.trim().to_owned()) {
            println!("Yes");
        } else {
            println!("No");
        }
        buffer.clear();
        if index == n {
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_str() {
        assert_eq!(check_str(&mut "([])".to_string()), true);
        assert_eq!(check_str(&mut "(([()])))".to_string()), false);
        assert_eq!(check_str(&mut "([()[]()])()".to_string()), true);
    }
}
