//https://www.hackerrank.com/challenges/mini-max-sum/problem

use std::io;
fn main() {
    // variable declaration
    let mut num_str = String::new();

    // read variables
    io::stdin().read_line(&mut num_str).ok().expect("read error");
    let nums : Vec<u64> = num_str
                        .split(' ')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse().unwrap())
                        .collect();

    let max = nums.iter().fold(nums[0], |acc, &n| if acc > n {acc} else {n});
    let min = nums.iter().fold(nums[0], |acc, &n| if acc < n {acc} else {n});


    // First tuple argument is a boolean indicating whether we've already skipped a number.
    // If there are two MAX or MIN values, then we will only skip the first one and not the last one.
    let small_sum = nums.iter().fold((false, 0), |acc, &n|
        if n == max && !acc.0 {
            (true, acc.1)
        } else {
            (acc.0, acc.1 + n)
        });
    let big_sum = nums.iter().fold((false, 0), |acc, &n|
        if n == min && !acc.0 {
            (true, acc.1)
        } else {
            (acc.0, acc.1 + n)
        });

    println!("{} {}", small_sum.1, big_sum.1);
}
