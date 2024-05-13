/*
--- Part Two ---

Now, given the same instructions, find the position of the first character that causes him to enter the basement (floor -1). The first character in the instructions has position 1, the second character has position 2, and so on.

For example:

    ) causes him to enter the basement at character position 1.
    ()()) causes him to enter the basement at character position 5.

What is the position of the character that causes Santa to first enter the basement?
*/

use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    File::open("assets/01.txt").unwrap().read_to_string(&mut input).unwrap();

    let begin_time = Instant::now();
    let awn: Result<u16, String> = primus(&input);

    let elp = begin_time.elapsed();

    println!("{} | {:.2?}", awn.unwrap() + 1, elp);
}

fn primus(input: &str) -> Result<u16, String> {
    let mut count: i32 = 0;
    let mut i: u16 = 0;
    for c in input.bytes() {
        match c {
            b'(' => count += 1,
            b')' => count -= 1,
            _ => (),
        }
        if count == -1 {
            return Ok(i);
        }
        i += 1;
    }
    Err("never goes to basement".to_string())
}
