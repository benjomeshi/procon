//https://www.hackerrank.com/challenges/birthday-cake-candles/problem

use std::io;
fn main() {
    // variable declaration
    let mut num_str = String::new();
    let mut nums_str = String::new();

    // read variables
    io::stdin().read_line(&mut num_str).ok().expect("read error");
    io::stdin().read_line(&mut nums_str).ok().expect("read error");

    let nums : Vec<u64> = nums_str
                        .split(' ')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse().unwrap())
                        .collect();


    let max = nums.iter().fold(1, |acc, &n| if n > acc {n} else {acc});

    let count_of_max = nums.iter().fold(0, |acc, &n| if n == max {acc+1} else {acc});

    println!("{}", count_of_max);
}
