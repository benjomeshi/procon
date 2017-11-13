//https://www.hackerrank.com/challenges/simple-array-sum/problem

use std::io;
fn main() {
    // variable declaration
    let mut _num_str_1 = String::new();
    let mut _num_str_2 = String::new();

    // read variables
    io::stdin().read_line(&mut _num_str_1).ok().expect("read error");
    io::stdin().read_line(&mut _num_str_2).ok().expect("read error");

    // parse integers
    let nums : Vec<i32> = _num_str_2
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let mut sum : i32 = 0;

    for n in nums{
        sum += n;
    }

    println!("{}", sum);
}
