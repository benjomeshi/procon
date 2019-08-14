//https://www.hackerrank.com/challenges/simple-array-sum/problem

use std::io;
fn main() {
    // variable declaration
    let mut _num_str_1 = String::new();

    // read variables
    io::stdin().read_line(&mut _num_str_1).ok().expect("read error");

    let num_rows : i32 = _num_str_1.trim().parse().unwrap();

    let mut primary_diag = vec![];
    let mut secondary_diag = vec![];

    for i in 0..num_rows{
        let mut row_string = String::new();
        io::stdin().read_line(&mut row_string).ok().expect("read error");

        let row : Vec<i32> = row_string
                                    .split(' ')
                                    .map(|s| s.trim())
                                    .filter(|s| !s.is_empty())
                                    .map(|s| s.parse().unwrap())
                                    .collect();

        primary_diag.push(row[i as usize]);
        let index_from_back : usize = (num_rows-1-i) as usize;
        secondary_diag.push(row[index_from_back]);
    }

    let primary_sum : i32 = primary_diag.iter().fold(0, |s, &b| s+b);
    let secondary_sum : i32 = secondary_diag.iter().fold(0, |s, &b| s+b);

    println!("{}", (primary_sum - secondary_sum).abs());
}
