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
        x:chars,
    }
    let x = x
        .iter()
        .map(|&c| if c == '1' { 1 } else { 0 })
        .collect::<Vec<_>>();
    let pc: isize = x.iter().fold(0, |s, v| s + v);
    let mut ans = vec![0; n];
    for b in 0..2 {
        let npc = if b == 0 { pc + 1 } else { pc - 1 };
        if npc <= 0 {
            continue;
        }
        let mut r0 = 0;
        for xi in x.iter() {
            r0 = (r0 * 2) % npc;
            r0 += xi;
        }
        let mut k = 1;
        for (i, &xi) in x.iter().enumerate().rev() {
            if xi == b {
                let r = if b == 0 {
                    (r0 + k) % npc
                } else {
                    (r0 + npc - k) % npc
                };
                ans[i] = f(r as u32) + 1;
            }
            k = (k * 2) % npc;
        }
    }

    let mut out = String::with_capacity(n * 2);
    for a in ans.iter() {
        out.push_str(&format!("{}\n", a));
    }
    out
}

fn f(x: u32) -> u32 {
    if x > 0 {
        f(x % popcount(x)) + 1
    } else {
        0
    }
}

fn popcount(x: u32) -> u32 {
    x.count_ones()
}
test! {
    include_str!("d1_in.txt") => include_str!("d1_out.txt"),
    include_str!("d2_in.txt") => include_str!("d2_out.txt"),
    include_str!("d3_in.txt") => include_str!("d3_out.txt"),
}
