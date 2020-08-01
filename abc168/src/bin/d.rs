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
            out!(concat!($fmt,"\n"));
        }};
        ($fmt:expr, $($args:tt)*) => {{
            out!(concat!($fmt, "\n"), $($args)*);
        }};
    }
    input! {
        source = src,
        n:usize,
        m:usize,
        ab:[(usize1,usize1);m],
    }
    let mut to = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        to[a].push(b);
        to[b].push(a);
    }

    let mut q = VecDeque::new();
    let mut dist = vec![usize::MAX; n];
    let mut pre = vec![usize::MAX; n];
    dist[0] = 0;
    q.push_front(0);

    while !q.is_empty() {
        let now = q.pop_back();
        if let Some(now) = now {
            for &next in to[now].iter() {
                if dist[next] != usize::MAX {
                    continue;
                }
                dist[next] = dist[now] + 1;
                pre[next] = now;
                q.push_front(next);
            }
        }
    }

    outln!("Yes");
    for pi in pre[1..].iter() {
        outln!("{}", pi + 1);
    }
}
test! {
    include_str!("d1_in.txt")=> include_str!("d1_out.txt"),
    include_str!("d2_in.txt")=> include_str!("d2_out.txt"),

}
