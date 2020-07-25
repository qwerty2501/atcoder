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
    let mut lmi = None;
    for i in (0..n).rev() {
        if is_mountain(&a, i) {
            lmi = Some(i);
            break;
        }
    }

    let mut amount = 1000;
    let mut stock = None;
    if let Some(lmi) = lmi {
        for i in 0..=lmi {
            if is_valley(&a, i) {
                if let None = stock {
                    let c = a[i];
                    let buy_count = amount / c;
                    stock = Some(buy_count);
                    amount -= c * buy_count;
                }
            }
            if is_mountain(&a, i) {
                if let Some(buy_count) = stock {
                    let c = a[i];
                    amount += c * buy_count;
                    stock = None;
                }
            }
        }
    }

    amount.to_string()
}

fn is_mountain(a: &[usize], i: usize) -> bool {
    let mut before = 0;
    if i > 0 {
        if let Some(b) = a.get(i - 1) {
            before = *b;
        }
    }
    let mut after = 0;
    if let Some(af) = a.get(i + 1) {
        after = *af;
    }
    let current = a[i];
    before <= current && after < current
}

fn is_valley(a: &[usize], i: usize) -> bool {
    let mut before = usize::MAX;
    if i > 0 {
        if let Some(b) = a.get(i - 1) {
            before = *b;
        }
    }
    let mut after = 0;
    if let Some(af) = a.get(i + 1) {
        after = *af;
    }
    let current = a[i];
    before >= current && after > current
}
test! {
    include_str!("d1_in.txt") => "1685",
    include_str!("d2_in.txt") => "1000",
    include_str!("d3_in.txt") => "1216",
}
