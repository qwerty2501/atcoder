#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::iter::*;
#[allow(unused_imports)]
use std::*;

macro_rules! test {
    ($($input:expr => $output:expr),* $(,)*) => {
#[test]
        fn solve_test() {
            $(
                assert_eq!(solve($input), $output);
            )*
        }
    };
}
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

macro_rules! stdin {
    () => {{
        use std::io::Read;
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    }};
}

fn main() {
    println!("{}", solve(&stdin!()));
}
fn solve(src: &str) -> String {
    input! {
        source = src,
        n:usize,
        a:[usize;n],
        q:usize,
        bcs:[(usize,usize);q],
    }
    let mut count_map = HashMap::<usize, usize>::with_capacity(n);

    for ai in a.iter() {
        count_map.insert(*ai, count_map.get(ai).unwrap_or(&0) + 1);
    }

    let mut s = Vec::with_capacity(q + 1);
    s.push(
        count_map
            .iter()
            .fold(0 as u64, |s, (k, v)| s + (k * v) as u64),
    );
    for (b, c) in bcs.iter() {
        if let Some(aic) = count_map.get(b) {
            let aic = aic.clone();
            let cc = (*c * aic) as u64;
            let bc = (*b * aic) as u64;
            let current = *s.last().unwrap();
            s.push(current + cc - bc);
            count_map.insert(*c, count_map.get(c).unwrap_or(&0) + aic);
            count_map.remove(b);
        } else {
            s.push(*s.last().unwrap());
        }
    }

    s[1..]
        .iter()
        .fold(String::with_capacity(q * 2), |mut s, v| {
            s.push_str(&format!("{}\n", v));
            s
        })
}
test! {
    include_str!("d1_in.txt") => include_str!("d1_out.txt"),
    include_str!("d2_in.txt") => include_str!("d2_out.txt"),
    include_str!("d3_in.txt") => include_str!("d3_out.txt")
}
