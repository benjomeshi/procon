//https://atcoder.jp/contests/abs/tasks/abc081_b
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
    let mut a = read!([usize]);

    let mut i: usize = 0;
    while a.iter().all(|x| x % 2 == 0) {
        i += 1;
        a = a.iter().map(|x| x / 2).collect();
    }

    println!("{}", i);
}
