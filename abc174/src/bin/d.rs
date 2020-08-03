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

fn main() {
    let mut out = std::io::stdout();

    solve(&stdin!(), &mut out);
}
fn solve<W: std::io::Write>(src: &str, out: &mut W) {
    macro_rules! out {
        ($fmt:expr) => {{
            let _ = out.write_fmt(format_args!($fmt));
        }};
        ($fmt:expr, $($args:tt)*) => {{
            let _ = out.write_fmt(format_args!($fmt, $($args)*));
        }};
    }
    macro_rules! outln {
        ($fmt:expr) => {{
            out!(concat!($fmt,"
"));
        }};
        ($fmt:expr, $($args:tt)*) => {{
            out!(concat!($fmt, "
"), $($args)*);
        }};
    }
    input! {
        source = src,
        n:usize,
        C:chars,
    }
    let mut count = 0;
    let mut end = None;
    let afters = C;
    let mut red_points = VecDeque::new();
    for (i, &a) in afters.iter().enumerate().rev() {
        if a == 'R' {
            if end == None {
                end = Some(i);
            }
            red_points.push_front(i);
        }
    }
    if let Some(end) = end {
        let mut afters = afters[..=end].iter().collect::<Vec<_>>();
        let mut i = 0;
        let l = afters.len();
        while i + 1 < l {
            let now = afters[i];
            if *now == 'W' {
                if let Some(red_point) = red_points.pop_back() {
                    if i < red_point {
                        afters[i] = afters[red_point];
                        afters[red_point] = now;
                        count += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            i += 1;
        }
    }
    out!("{}", count);
}
test! {
    include_str!("d1_in.txt")=>"2",
    include_str!("d2_in.txt")=>"0",
    include_str!("d3_in.txt")=>"3",

}
