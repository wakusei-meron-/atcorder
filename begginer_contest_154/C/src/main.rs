use std::collections::HashMap;
use std::io::*;

fn main() {
    let out = stdout();
    let mut out = out.lock();
    let mut first_line = String::new();
    stdin().read_line(&mut first_line).unwrap();
    let n: usize = first_line.trim().parse().unwrap();

    let mut second_line = String::new();
    stdin().read_line(&mut second_line).unwrap();
    let v: Vec<&str> = second_line.split_whitespace().collect();
    let mut map = HashMap::new();
    for i in 0..n {
        if map.contains_key(v[i]) {
            write!(out, "NO").unwrap();
            return;
        }
        map.insert(v[i], "");
    }
    write!(out, "YES").unwrap();
}
