//https://www.hackerrank.com/challenges/staircase/problem

use std::io;
fn main() {
    // variable declaration
    let mut num_str = String::new();

    // read variables
    io::stdin().read_line(&mut num_str).ok().expect("read error");
    let num : i32 = num_str.trim().parse().unwrap();

    for i in  1..num+1{
        for _ in 0..(num-i){
            print!(" ");
        }
        for _ in 0..(i){
            print!("#");
        }
        println!("");
    }
}
