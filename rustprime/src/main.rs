use text_io::read;
use std::time::{Instant};

fn main() {
    println!("Welcome to the prime number checker.\nPlease enter a number below to check if it is prime.");
    let p: i64 = read!();
    //Counting algorithm runtime duration
    let time = Instant::now();
    prime_check(p);
    println!("The function checker took {} seconds to check.", time.elapsed().as_secs());
}
//Prime number checker function
fn prime_check(n: i64) {
    let mut count = 2i64;

    loop {
        if count == n {
            println!("Number is prime.");
            break;
        }

        if n % count == 0 {
            println!("Number is not prime");
            break;
        }
        
        count += 1;
    }
}