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
        n:chars,
    }
    let mut k = n.len();

    let mut ans_tmp: HashMap<isize, isize> = HashMap::new();
    let mut ans = -1;
    for kc in 0..k {
        let r = dns(n.clone(), kc, 0, &mut ans_tmp);
        if r >= 0 && ans >= 0 {
            ans = min(ans, r);
        } else {
            ans = max(ans, r);
        }
        if ans >= 0 {
            break;
        }
    }
    out!(ans);
}

fn dns(n: Vec<char>, mut kc: usize, ans: isize, ans_tmp: &mut HashMap<isize, isize>) -> isize {
    let v = n.iter().collect::<String>().parse::<isize>().unwrap();
    let va = ans_tmp.get(&v);
    if va.is_some() {
        *va.unwrap()
    } else if v % 3 == 0 {
        ans_tmp.insert(v, ans);
        ans
    } else if kc == 0 {
        -1
    } else {
        let mut tans = -1;
        kc -= 1;
        for i in 0..n.len() {
            let mut tn = n.clone();
            tn.remove(i);
            let r = dns(tn, kc, ans + 1, ans_tmp);
            if r >= 0 && tans >= 0 {
                tans = min(tans, r);
            } else {
                tans = max(tans, r);
            }
        }
        tans
    }
}
test! {}
