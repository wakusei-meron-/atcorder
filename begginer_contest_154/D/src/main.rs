// 等差数列の和は最小と最大の和÷2
// 整数で扱いたい場合はx2しておくと良いかも
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

fn max(a: f64, b: f64) -> f64 {
    if a >= b {
        return a;
    }
    return b;
}

fn main() {
    input! {
      n: usize,
      k: usize,
      mat: [f64; n],
    }
    let mut p_map = std::collections::HashMap::new();

    // create map
    let mut mu_max = 0.;
    for i in 0..n {
        let key = mat[i].to_string();
        if p_map.contains_key(&key) {
            continue;
        }
        let mut temp_mu = 0.;
        let p: f64 = 1. / mat[i];
        for ii in 1..mat[i] as i32 + 1 {
            temp_mu += p * (ii as f64);
        }
        p_map.insert(key, temp_mu);
    }

    // calc max
    let mut mu = 0.;
    let mut mu_max = 0.;
    for i in 0..n {
        let key = mat[i].to_string();
        mu = mu + p_map.get(&key).unwrap();
        if i + 1 >= k {
            mu_max = max(mu_max, mu);

            let key = mat[i + 1 - k].to_string();
            mu = mu - p_map.get(&key).unwrap();
        }
    }
    println!("{}", mu_max);
}
