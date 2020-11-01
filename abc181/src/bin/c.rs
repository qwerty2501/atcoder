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
                {
                    let mut out = Vec::<u8>::new();
                    solve($input,&mut out);
                    let out_str =str::from_utf8(&out).unwrap();
                    assert_eq!(&out_str, &$output);
                }
            )*
        }
    };
}
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
#[allow(unused_mut)]
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
    ($iter:expr, isize1) => {
        read_value!($iter, isize) - 1
    };
    ($iter:expr,str) => {
        $iter.next().unwrap()
    };

    ($iter:expr, $t:tt) => {
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

#[allow(dead_code)]
fn to_yesno(yesno: bool) -> &'static str {
    if yesno {
        "Yes"
    } else {
        "No"
    }
}

fn main() {
    let mut out = std::io::stdout();

    solve(&stdin!(), &mut out);
}

#[allow(unused_mut)]
fn solve<W: std::io::Write>(src: &str, out: &mut W) {
    #[allow(unused_macros)]
    macro_rules! out {
        ($arg:expr) => {{
            let _ = out.write_fmt(format_args!("{}",$arg));
        }};
        ($fmt:expr, $($args:tt)*) => {{
            let _ = out.write_fmt(format_args!($fmt, $($args)*));
        }};
    }
    #[allow(unused_macros)]
    macro_rules! outln {
        ($arg:expr) => {{
            out!("{}\n",$arg);
        }};
        ($fmt:expr, $($args:tt)*) => {{
            out!(concat!($fmt, "\n"), $($args)*);
        }};
    }

    input! {
        source = src,
        n:usize,
        xy_s:[(isize,isize);n],
    }

    for (i, (x, y)) in xy_s.iter().enumerate() {
        for (j, (x2, y2)) in xy_s.iter().enumerate() {
            if i != j {
                let r = calc_r(y - y2, x - x2);
                for (k, (x3, y3)) in xy_s.iter().enumerate() {
                    if i != k && j != k {
                        let r1 = calc_r(y - y3, x - x3);
                        let r2 = calc_r(y2 - y3, x2 - x3);
                        let d_ans =
                            r == r1 && r == r2 && r.is_some() && r1.is_some() && r2.is_some();
                        let x_ans = x == x2 && x == x3;
                        let y_ans = y == y2 && y == y3;
                        if d_ans || x_ans || y_ans {
                            out!(to_yesno(true));
                            return;
                        }
                    }
                }
            }
        }
    }
    out!(to_yesno(false));
}
fn calc_r(x: isize, y: isize) -> Option<f64> {
    if x != 0 {
        Some(y as f64 / x as f64)
    } else {
        None
    }
}
test! {}
