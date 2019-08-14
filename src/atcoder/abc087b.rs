//https://atcoder.jp/contests/abs/tasks/abc087_b

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
    let a = read!(usize);
    let b = read!(usize);
    let c = read!(usize);
    let x = read!(usize);

    let mut n = 0;

    for i in 0..a + 1 {
        for j in 0..b + 1 {
            for k in 0..c + 1 {
                if 500 * i + 100 * j + 50 * k == x {
                    n += 1;
                }
            }
        }
    }

    println!("{}", n)
}
