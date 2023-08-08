use std::collections::{BinaryHeap, HashMap};
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    let n = buffer.trim().parse::<usize>().unwrap();
    let mut names_map: HashMap<String, u32> = HashMap::with_capacity(n as usize);
    for _ in 0..n {
        buffer.clear();
        if stdin.read_line(&mut buffer).is_err() {
            break;
        }
        let name = buffer.clone().trim().to_string();
        *names_map.entry(name).or_default() += 1;
    }
    let mut b_heap = BinaryHeap::with_capacity(n as usize);
    for (name, count) in names_map {
        b_heap.push((count, name));
    }
    let (count, name) = b_heap.pop().unwrap();
    println!("{name} {count}");
    Ok(())
}
