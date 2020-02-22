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

fn main() {
    input!(
      n: i64,
      mat: [i64; n],
    );
    let mat2 = mat.clone();
    let mut w = 0;
    for x in mat {
        w += x
    }
    let p = w/n;
    let p2 = w/n + 1;

    let mut cost = 0;
    let mut cost2 = 0;
    for x in mat2 {
        cost += (x - p) * (x - p);
        cost2 += (x - p2) * (x - p2);
    }

    if cost < cost2 {
        println!("{}", cost);
    }else {
        println!("{}", cost2);
    }
}