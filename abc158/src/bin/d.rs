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
    let lines = src.split('\n');
    let lines = lines.collect::<Vec<_>>();
    let mut s = lines[0].chars().collect::<VecDeque<_>>();
    input! {
        source = lines[1],
        q:usize
    }
    let mut reversed = false;
    for &line in lines[2..].iter() {
        if line == "" {
            continue;
        }
        let strs = line.split_whitespace().collect::<Vec<_>>();
        input! {
            source = strs[0],
            ti:usize,
        }
        match ti {
            1 => {
                reversed = !reversed;
            }
            2 => {
                input! {
                    source = strs[1],
                    fi:usize,
                }
                input! {
                    source = strs[2],
                    ci:char,
                }
                let push_front = ((fi == 1) && !reversed) || ((fi == 2) && reversed);
                if push_front {
                    s.push_front(ci);
                } else {
                    s.push_back(ci);
                }
            }
            v => panic!("invalid value:{}", v),
        }
    }
    let r = if reversed {
        to_display(s.iter().rev(), s.len())
    } else {
        to_display(s.iter(), s.len())
    };

    r
}

fn to_display<'a, T: Iterator<Item = &'a char>>(itr: T, l: usize) -> String {
    itr.fold(String::with_capacity(l), |mut s, c| {
        s.push(*c);
        s
    })
}
test! {
    include_str!("d1_in.txt") => "cpa",
    include_str!("d2_in.txt") => "aabc",
    include_str!("d3_in.txt") => "xy",
}
