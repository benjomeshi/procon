//https://www.hackerrank.com/challenges/time-conversion/problem

use std::io;
fn main() {
    // variable declaration
    let mut time_str = String::new();

    // read variables
    io::stdin().read_line(&mut time_str).ok().expect("read error");

    let times : Vec<&str> = time_str
                                .split(':')
                                .map(|s| s.trim())
                                .collect();
    let am_pm : String = times[2].chars().skip(2).take(2).collect();

    let hour : i32 = times[0].parse().unwrap();
    let hour_24 = if hour == 12 {
        if am_pm == "PM" {12} else {0}
    }else{
        if am_pm == "PM" {hour + 12} else {hour}
    };

    let rest_of_time : String = time_str.chars().skip(2).take(6).collect();

    println!("{:02}{}", hour_24, rest_of_time.as_str());
}
