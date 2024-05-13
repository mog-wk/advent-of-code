/*
 --- Part Two ---

 The next year, to speed up the process, Santa creates a robot version of himself, Robo-Santa, to
 deliver presents with him.

 Santa and Robo-Santa start at the same location (delivering two presents to the same starting
 house), then take turns moving based on instructions from the elf, who is eggnoggedly reading from
 the same script as the previous year.

 This year, how many houses receive at least one present?

For example:

    ^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
    ^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
    ^v^v^v^v^v now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.

    => 2341
*/

use std::fs::File;
use std::io::Read;

use std::collections::HashSet;
use std::time::{Duration, Instant};

fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    File::open("assets/03.txt").unwrap().read_to_string(&mut input).unwrap();

    let anw = hash_set_impl(&input);

    println!("{} | {:.2?}", anw.0, anw.1);
}

fn hash_set_impl(input: &str) -> (usize, Duration) {
    let bench = Instant::now();
    let mut visited = HashSet::new();
    visited.insert( (0, 0) );
    let mut loc_a: (i32, i32) = (0, 0);
    let mut loc_b: (i32, i32) = (0, 0);

    let mut c = 0;
    for direction in input.bytes() {
        let loc = if c % 2 == 0 {
            &mut loc_a
        } else {
            &mut loc_b
        };
        match direction {
            b'>' => loc.0 += 1,
            b'<' => loc.0 -= 1,
            b'^' => loc.1 += 1,
            b'v' => loc.1 -= 1,
            _ => (),
        }
        c += 1;
        println!("{:?} {}", loc, c);
        visited.insert(*loc);
    }
    (visited.len(), bench.elapsed())
}
