use std::io::*;

fn main() {
    let mut first_line = String::new();
    stdin().read_line(&mut first_line).unwrap();
    let target_len = first_line.trim().len();

    let out = stdout();
    let mut out = out.lock();

    for x in 0..target_len {
        write!(out, "x");
    }
}
