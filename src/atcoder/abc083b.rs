//https://atcoder.jp/contests/abs/tasks/abc083_b

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
    let (n, a, b) = read!(u32, u32, u32);

    let mut sums: Vec<u32> = Vec::new();

    for m in 0..n + 1 {
        let sum: u32 = m.to_string().chars().map(|x| x.to_digit(10).unwrap()).sum();
        if sum >= a && sum <= b {
            sums.push(m);
        }
    }

    let sum: u32 = sums.iter().sum();

    println!("{}", sum);
}
