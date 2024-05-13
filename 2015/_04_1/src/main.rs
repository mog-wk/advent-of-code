/*
--- Day 4: The Ideal Stocking Stuffer ---
Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts for all the
economically forward-thinking little girls and boys.

To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes. The
input to the MD5 hash is some secret key (your puzzle input, given below) followed by a number in
decimal. To mine AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1,
2, 3, ...) that produces such a hash.

For example:

If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts
with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting
with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....

    key = "iwrupvqb";

    => 346386
    => 9958218
*/

mod lib;
use lib::md5;

use std::time::Instant;

// TODO make use of multiple threads for speeeed

fn main() {
    println!("Hello, world!");
    let key = "iwrupvqb";

    let mut counter: u64 = 30000;

    let bench = Instant::now();

    let zeros = 6_u8;

    let mut fives = vec![];

    // brute force
    'run: loop {

        let id = handle_id(counter);

        let attempt = md5(&(String::from(key) + &id));

        let mut result: u8 = 0;

        for c in attempt[0..zeros as usize].bytes() { 
            if c == b'0' {
                result += 1;
            }
        }
        println!("{:04} => {} {:?} {}", counter, id, attempt, result);
        if result == 5 {
            fives.push(format!("{}: {}", id, attempt));
            break 'run
        } else if result == 6 {
            println!("{}-{}", key, id);
            break 'run
        }
        counter += 1;
    }

    println!("{:?}", fives);
    println!("{:.2?}", bench.elapsed());
}

fn handle_id(n: u64) -> String {
    let bias = 10;
    for i in 0..bias {
        if n < 10_u64.pow(i) {
            return format!("{}", n);
        }
    }
    String::new()
}


#[cfg(test)]
mod test {
    use crate::md5;
    #[test]
    fn test_md5_abcd() {
        let digest = md5(&"abcdef609043");

        assert!(digest == "000001dbbfa3a5c83a2d506429c7b00e");
    }
    #[test]
    fn test_md5_pqrstuv() {
        let digest = md5(&"pqrstuv1048970");

        assert!(digest == "000006136ef2ff3b291c85725f17325c");
    }
}
