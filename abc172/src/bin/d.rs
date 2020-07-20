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
        n:u64,
    }
    let mut sum = 0;
    for x in 1..=n {
        sum += x * g(n, x);
    }
    sum.to_string()
}

fn g2(n: u64) -> u64 {
    let mut i = 1;
    let mut nc = 1;
    while n >= i {
        if n / i - 1 >= 1 {
            nc += 1;
        }
        i *= i.pow(2);
    }
    n * nc
}
fn g(n: u64, d: u64) -> u64 {
    (n / d) * (2 + ((n / d) - 1) * d) / 2
}

fn f(x: u64, count_map: &HashMap<u64, u64>) -> u64 {
    let mut n = x;
    let mut c = 1;
    let mut a = 2;
    while n >= a * a {
        if n % a == 0 {
            let mut ex = 0;
            while {
                ex += 1;
                n /= a;
                n % a == 0
            } {}
            c *= ex + 1;
        } else {
            a += 1;
        }
    }
    if n != 1 {
        c *= 2;
    }
    c
}
test! {
    "1" => "1",
    "2" => "5",



    /*
    "4" => "23",
    "5" => "33",
    */
    /*
    "6" => "57",
    "100" => "26879",
*/

//    "10000000" => "838627288460105",
}
