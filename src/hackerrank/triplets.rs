//https://www.hackerrank.com/challenges/compare-the-triplets/problem

use std::io;
fn main() {
    // variable declaration
    let mut alice_points_str = String::new();
    let mut bob_points_str = String::new();

    // read variables
    io::stdin().read_line(&mut alice_points_str).ok().expect("read error");
    io::stdin().read_line(&mut bob_points_str).ok().expect("read error");

    // parse integers
    let alice : Vec<i32> = alice_points_str
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let bob : Vec<i32> = bob_points_str
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();


    let mut alice_score = 0;
    let mut bob_score = 0;

    for it in alice.iter().zip(bob.iter()){
        let (a,b) = it;
        if a > b {
            alice_score += 1;
        }
        if b > a {
            bob_score += 1;
        }
    }

    // print the sum
    // Hint: Type println!("{}", _num_1 + _num_2); below
    println!("{} {}", alice_score, bob_score);
}
