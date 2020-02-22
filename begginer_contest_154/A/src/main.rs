use std::io::*;

fn main() {
    let mut first_line = String::new();
    stdin().read_line(&mut first_line).unwrap();
    let v: Vec<&str> = first_line.split_whitespace().collect();
    let s = v[0];
    let t = v[1];

    let mut second_line = String::new();
    stdin().read_line(&mut second_line).unwrap();
    let v2: Vec<&str> = second_line.split_whitespace().collect();
    let mut s_num: i64 = match v2[0].trim().parse() {
        Ok(n) => n,
        Err(_) => 0,
    };
    let mut t_num: i64 = match v2[1].trim().parse() {
        Ok(n) => n,
        Err(_) => 0,
    };
    let mut third_line = String::new();
    stdin().read_line(&mut third_line).unwrap();
    let target = third_line.trim();

    if target == t {
        t_num -= 1;
    } else {
        s_num -= 1;
    }

    let out = stdout();
    let mut out = out.lock();
    write!(out, "{} {}", s_num, t_num);
}

