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

#[allow(unused_mut)]
fn solve<W: std::io::Write>(src: &str, out: &mut W) {
    #[allow(unused_macros)]
    macro_rules! out {
        ($fmt:expr) => {{
            let _ = out.write_fmt(format_args!($fmt));
        }};
        ($fmt:expr, $($args:tt)*) => {{
            let _ = out.write_fmt(format_args!($fmt, $($args)*));
        }};
    }
    #[allow(unused_macros)]
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
        h:usize,
        _w:usize,
        ch:usize1,
        cw:usize1,
        dh:usize1,
        dw:usize1,
        s:[chars;h],
    }
    let ch = ch as isize;
    let cw = cw as isize;
    let dh = dh as isize;
    let dw = dw as isize;
    let mut que = VecDeque::new();
    que.push_back((ch, cw, 0));
    let mut exists = HashSet::new();

    while let Some((h, w, wc)) = que.pop_back() {
        if h == dh && w == dw {
            out!("{}", wc);
            return;
        }
        if !can_walk(h, w, &s, &exists) {
            continue;
        }
        exists.insert((h, w));

        for &(h, w) in [(h + 1, w), (h, w + 1), (h - 1, w), (h, w - 1)].iter() {
            if can_walk(h, w, &s, &exists) {
                que.push_back((h, w, wc));
            }
        }
        for wh in -2..=2 {
            for ww in -2..=2 {
                let (wh, ww) = (h + wh, w + ww);
                if can_walk(wh, ww, &s, &exists) {
                    que.push_front((wh, ww, wc + 1));
                }
            }
        }
    }
    out!("{}", -1);
}
fn can_walk(h: isize, w: isize, s: &[Vec<char>], exists: &HashSet<(isize, isize)>) -> bool {
    h >= 0
        && w >= 0
        && h < s.len() as isize
        && w < s[h as usize].len() as isize
        && s[h as usize][w as usize] == '.'
        && !exists.contains(&(h, w))
}
test! {
    include_str!("d1_in.txt") => "1",
}
