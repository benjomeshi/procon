//https://atcoder.jp/contests/abs/tasks/arc065_a

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
    let string = read!(String);
    let mut s = string.as_str();

    let mut valid = true;

    while !s.is_empty() {
        if s.len() < 5 {
            valid = false;
            break;
        }
        let (fst, lst) = s.split_at(5);
        s = lst;
        match fst {
            "dream" => {
                // end of string
                if s.is_empty() {
                    break;
                }

                if s.starts_with("er") && !s.starts_with("erase") {
                    let (_, rest) = s.split_at(2);
                    s = rest;
                }

                if !s.starts_with('e') && !s.starts_with('d') && !s.is_empty() {
                    valid = false;
                    break;
                }
            }

            "erase" => {
                if s.starts_with('r') {
                    let (_, rest) = s.split_at(1);
                    s = rest;
                }

                if !s.starts_with('e') && !s.starts_with('d') && !s.is_empty() {
                    valid = false;
                }
            }

            _ => {
                valid = false;
            }
        }
    }

    println!("{}", if valid { "YES" } else { "NO" })
}
