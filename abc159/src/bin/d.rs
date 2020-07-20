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
    }
    let mut c_map = HashMap::<usize, usize>::with_capacity(n);
    for ai in a.iter() {
        let mut c = *c_map.get(ai).unwrap_or(&0);
        c += 1;
        c_map.insert(*ai, c);
    }
    let mut sum = 0;
    for (_, &v) in c_map.iter() {
        sum += c_2(v);
    }
    let mut answers = String::with_capacity(n * 2);
    for ai in a.iter() {
        let c = c_map[ai];
        let b = c_2(c);
        let c = c_2(c - 1);
        let ans = sum - (b - c);
        answers.push_str(&format!("{}\n", ans));
    }
    answers
}

fn c_2(v: usize) -> usize {
    if v > 1 {
        (v * (v - 1)) / 2
    } else {
        0
    }
}
test! {
    include_str!("d1_in.txt") => include_str!("d1_out.txt"),
    include_str!("d2_in.txt") => include_str!("d2_out.txt"),
    include_str!("d3_in.txt") => include_str!("d3_out.txt"),
    include_str!("d4_in.txt") => include_str!("d4_out.txt"),
}
