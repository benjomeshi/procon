//http://judge.u-aizu.ac.jp/onlinejudge/submission.jsp#submit/ALDS1_1/C
use std::io;

fn main(){
    let mut n_str = String::new();

    // read
    io::stdin().read_line(&mut n_str).ok().expect("read error");

    let n: i64 = n_str.parse();
    let mut primes_count : i64 = 0;

    let mut x_str = String::new();
    let isPrime = createIsPrime();
    for _ in 0..n {
        io::stdin().read_line(&mut x_str).ok().expect("read error");

        let x: i64 = x_str.parse();
        primes_count = if isPrime(x) {primes_count+1}else{primes_count};
    }

    stdout().print_line(primes_count)

}

fn createIsPrime(){
    let mut prior_primes: Vec<i64> = vec![2,3,5,7,11,13,17,19,23,29,31];
    |x: i64| -> bool {
        if prior_primes.into_iter().any(|y| (x % y) == 0){
            return false;
        }else{
            let max_prime = prior_primes.into_iter().fold(prior_primes[0], |acc, &n| if acc > n {acc} else {n});
            for i in max_prime..(x/2){
                if x % i == 0{
                    return false
                }else{
                    return true
                }
            }
        }
    }
}
