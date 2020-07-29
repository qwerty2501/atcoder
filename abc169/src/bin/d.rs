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

fn div_game(mut n: usize) -> usize {
    let mut v = 2;
    let mut r = 0;

    let mut mark_map = HashSet::new();
    while n >= v * v {
        if n % v == 0 {
            if !mark_map.contains(&v) {
                let mut div_c = 1;
                let mut d = v;
                while n % d == 0 {
                    n /= d;
                    mark_map.insert(d);
                    div_c += 1;
                    d = v.pow(div_c);
                }
                r += div_c as usize - 1;
            } else {
                n /= v;
            }
        } else {
            v += 1;
        }
    }
    if n > 1 && !mark_map.contains(&n) {
        r += 1;
    }

    r
}
fn solve(src: &str) -> String {
    input! {
        source = src,
        n:usize,
    }
    div_game(n).to_string()
}
test! {

    "24" => "3",
    "1"=> "0",
    "64"=> "3",
    "1000000007"=>"1",
    "997764507000"=> "7",
    "4"=>"1",
    "32"=>"2",
    "16384" => "4",
    "32768" => "5",


}
