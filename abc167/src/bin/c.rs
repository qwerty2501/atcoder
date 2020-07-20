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
struct Book<'a> {
    c: usize,
    u_s: &'a [usize],
}
fn solve(src: &str) -> String {
    input! {
        source = src,
        n:usize,
        m:usize,
        x:usize,
        nums:[[usize;m+1];n],
    }
    let books = nums
        .iter()
        .map(|ns| Book {
            c: ns[0],
            u_s: &ns[1..ns.len()],
        })
        .collect::<Vec<_>>();
    let m_c = r_min(usize::MAX, x, vec![0; m], &books);
    (m_c as isize).to_string()
}

fn r_min(min_c: usize, x: usize, u_s: Vec<usize>, books: &[Book]) -> usize {
    if !books.is_empty() {
        let current_book = &books[0];
        let next_books = &books[1..books.len()];
        let not_read_current = r_min(min_c, x, u_s.clone(), next_books);
        let u_s = u_s
            .iter()
            .enumerate()
            .map(|(i, u)| current_book.u_s[i] + u)
            .collect::<Vec<_>>();
        let min_c = if min_c == usize::MAX { 0 } else { min_c } + current_book.c;
        let read_current = if u_s.iter().all(|&u| u >= x) {
            min_c
        } else {
            r_min(min_c, x, u_s, next_books)
        };

        min(not_read_current, read_current)
    } else if u_s.iter().all(|&u| u >= x) {
        min_c
    } else {
        usize::MAX
    }
}

test! {
    include_str!("c1_in.txt") => "120",
    include_str!("c2_in.txt") => "-1",
    include_str!("c3_in.txt") => "1067",
}
