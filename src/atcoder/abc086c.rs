//https://atcoder.jp/contests/abs/tasks/arc089_a

#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::collections::*;
use std::io::{stderr, stdin, BufRead, Write};

#[allow(dead_code)]
fn rl() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim_right().to_owned()
}

#[allow(unused_macros)]
macro_rules! read {
    ([$t:ty] ; $n:expr) =>
        ((0..$n).map(|_| read!([$t])).collect::<Vec<_>>());
    ($($t:ty),+ ; $n:expr) =>
        ((0..$n).map(|_| read!($($t),+)).collect::<Vec<_>>());
    ([$t:ty]) =>
        (rl().split_whitespace().map(|w| w.parse().unwrap()).collect::<Vec<$t>>());
    ($t:ty) =>
        (rl().parse::<$t>().unwrap());
    ($($t:ty),*) => {{
        let buf = rl();
        let mut w = buf.split_whitespace();
        ($(w.next().unwrap().parse::<$t>().unwrap()),*)
    }};
}

fn main() {
    let n = read!(usize);
    let stuff = read!(i64, i64, i64; n);

    let mut valid = true;
    let mut prev = (0, 0, 0);
    for state in stuff {
        let (tp, xp, yp) = prev;
        let (t, x, y) = state;

        let leftover_distance = (t - tp) - (x - xp).abs() - (y - yp).abs();
        valid = valid && leftover_distance >= 0 && leftover_distance % 2 == 0;

        prev = state;
    }

    println!("{}", if valid { "Yes" } else { "No" });
}
