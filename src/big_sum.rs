//https://www.hackerrank.com/challenges/a-very-big-sum/problem

use std::io;
fn main() {
    // variable declaration
    let mut n = String::new();
    let mut ns = String::new();

    // read variables
    io::stdin().read_line(&mut n).ok().expect("read error");
    io::stdin().read_line(&mut ns).ok().expect("read error");

    // parse integers
    let big_numbers : Vec<u64> = ns
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let sum : u64 = big_numbers.iter().fold(0, |acc, &b| acc + b);

    println!("{}", sum);
}
