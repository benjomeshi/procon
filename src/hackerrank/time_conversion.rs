//https://www.hackerrank.com/challenges/time-conversion/problem

use std::io;
fn main() {
    // variable declaration
    let mut n_str = String::new();
    let mut ns_str = String::new();
    let mut pq_str = String::new();

    // read variables
    io::stdin().read_line(&mut n_str).ok().expect("read error");
    io::stdin().read_line(&mut ns_str).ok().expect("read error");
    io::stdin().read_line(&mut pq_str).ok().expect("read error");

    let ns : Vec<i32> = ns_str.split(' ')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .map(|s| s.parse().unwrap())
                            .collect();
    let pq : Vec<i32> = pq_str.split(' ')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .map(|s| s.parse().unwrap())
                            .collect();

    let p : i32 = pq[0];
    let q : i32 = pq[1];

    fn min_a_minus_m(ns : &Vec<i32>, m : i32) -> i32 {
        return ns.iter().fold(std::i32::MAX, |min, &n| if min > (n-m).abs() {
            (n-m).abs()
        } else {
            min
        });
    }

    let mut max = (p, min_a_minus_m(&ns, p));
    let max_ns = ns.iter().fold(0, |max, &n| if max < n {n} else {max});
    for m in p..q+1{
        if m > max_ns {
            let min = min_a_minus_m(&ns, q);
            if max.1 < min{
                max = (q, min);
                break;
            }
        }

        let min = min_a_minus_m(&ns, m);
        if max.1 < min{
            max = (m, min);
        }
    }

    println!("{}", max.0);
}
