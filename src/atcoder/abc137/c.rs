//https://atcoder.jp/contests/abc137/tasks/abc137_c

#![allow(unused_imports)]

use std::cmp::{max, min, Ordering};
use std::collections::*;
use std::io::{stderr, stdin, BufRead, Write};

use std::iter::FromIterator;

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

fn sort_by_chars(a: &Vec<char>, b: &Vec<char>) -> Ordering {
    for (a_, b_) in a.iter().zip(b.iter()) {
        match a_.cmp(b_) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => (),
        }
    }
    Ordering::Equal
}

// special case of nCr when r=2
// i.e. \frac{n!}{(n-2)!2!)
// count (n) will also be > 0 so we can ignore that case, n=1 will be 0 like we want it to be 
fn combinations(n: u64) -> u64 {
    (n * (n - 1)) / 2
}

fn main() {
    let n = read!(u64);
    let strings: Vec<String> = read!(String; n);

    let chars: Vec<Vec<char>> = strings.iter().map(|s| s.chars().collect()).collect();

    let mut sorted_chars = Vec::new();

    for mut word in chars {
        word.sort();
        sorted_chars.push(word);
    }

    sorted_chars.sort_by(sort_by_chars);

    let mut comb = 0;
    let mut count = 1;
    let mut prev_word: Vec<char> = (0..10).map(|_| ' ').collect();

    for w in sorted_chars {
        if w == prev_word {
            count += 1;
        } else {
            comb += combinations(count);
            count = 1;
        }
        prev_word = w;
    }

    comb += combinations(count);

    println!("{}", comb)
}
