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
        m:usize,
        q:usize,
        abcd_s:[(usize1,usize1,usize,usize);q],
    }
    dfs(vec![], &abcd_s, n, m).to_string()
}

fn dfs(mut A: Vec<isize>, abcd_s: &[(usize, usize, usize, usize)], n: usize, m: usize) -> usize {
    if A.len() == n + 1 {
        let sum = abcd_s
            .iter()
            .filter(|&&(a, b, c, _)| A[b] - A[a] == c as isize)
            .fold(0, |s, (_, _, _, d)| s + d);
        return sum;
    }
    A.push(*A.last().unwrap_or(&1));

    let mut ans = 0;
    while *A.last().unwrap() <= m as isize {
        ans = max(ans, dfs(A.clone(), abcd_s, n, m));
        let l = A.last_mut().unwrap();
        *l += 1;
    }
    ans
}
test! {
    include_str!("c1_in.txt") => "110",
    include_str!("c2_in.txt") => "357500",
    include_str!("c3_in.txt") => "1",
}
