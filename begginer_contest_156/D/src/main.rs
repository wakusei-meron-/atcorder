macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}
use std::cmp;

fn find_min_max(x: &[i64]) -> (i64, i64) {
    let mut min: i64 = x[0];
    let mut max: i64 = x[0];
    let mut x_iter = x.iter();
    while let Some(i) = x_iter.next() {
        min = cmp::min(*i, min);
        max = cmp::max(*i, max);
    }
    return (min, max)
}

pub fn binom_knuth(n: usize, k: usize) -> usize {
    (0..n + 1)
        .rev()
        .zip(1..k + 1)
        .fold(1, |mut r, (n, d)| { r *= n; r /= d; r })
}

fn main() {
    input!(
      n: usize,
      a: usize,
      b: usize
    );
    let mut c = 0;
    let div = 10i64.pow(9) + 7;
    for i in 1..n+1 {
        if i == a || i == b {
            continue
        }
        c += binom_knuth(n, i);
        c = c % div as usize;
    }
    println!("{}", c);
}