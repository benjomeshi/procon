//https://www.hackerrank.com/challenges/plus-minus/problem

use std::io;
fn main() {
    // variable declaration
    let mut _num_str_1 = String::new();
    let mut array_string = String::new();

    // read variables
    io::stdin().read_line(&mut _num_str_1).ok().expect("read error");
    io::stdin().read_line(&mut array_string).ok().expect("read error");

    let num_items : f32 = _num_str_1.trim().parse().unwrap();

    let array : Vec<i32> = array_string
                                .split(' ')
                                .map(|s| s.trim())
                                .filter(|s| !s.is_empty())
                                .map(|s| s.parse().unwrap())
                                .collect();

    let count_positive = array.iter().fold(0, |acc, &a| if a > 0{ acc + 1 }else{ acc });
    let count_zero = array.iter().fold(0, |acc, &a| if a == 0{acc+1}else{acc});
    let count_negative = array.iter().fold(0, |acc, &a| if a < 0{acc+1}else{acc});

    println!("{}", (count_positive as f32)/num_items);
    println!("{}", (count_negative as f32)/num_items);
    println!("{}", (count_zero as f32)/num_items);
}
