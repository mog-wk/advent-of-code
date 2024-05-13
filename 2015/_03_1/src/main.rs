/*
   --- Day 3: Perfectly Spherical Houses in a Vacuum ---

   Santa is delivering presents to an infinite two-dimensional grid of houses.

   He begins by delivering a present to the house at his starting location, and then an elf at the
   North Pole calls him via radio and tells him where to move next. Moves are always exactly one
   house to the north (^), south (v), east (>), or west (<). After each move, he delivers another
   present to the house at his new location.

   However, the elf back at the north pole has had a little too much eggnog, and so his directions
   are a little off, and Santa ends up visiting some houses more than once. How many houses receive
   at least one present?

   For example:

   > delivers presents to 2 houses: one at the starting location, and one to the east.
   ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
   ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.

    => 2081
*/

use std::fs::File;
use std::io::Read;

use std::time::Instant;
use std::time::Duration;
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    File::open("assets/03.txt").expect("failed to read file").read_to_string(&mut input).unwrap();

    let set_answer = set_inpl(&input);
    println!("hash_set: {} | {:.2?}", set_answer.0, set_answer.1);
    let vec_answer = vec_impl(&input);
    println!("vec:      {} | {:.2?}", vec_answer.0, vec_answer.1);
}

fn set_inpl(input: &str) -> (usize, Duration) {
    let bench = Instant::now();

    let mut loc_map: HashSet<(i32, i32)> = HashSet::new();
    let mut loc: (i32, i32) = (0, 0);

    for direction in input.bytes() {
        loc_map.insert(loc);
        match direction {
            b'>' => loc.0 += 1,
            b'<' => loc.0 -= 1,
            b'v' => loc.1 += 1,
            b'^' => loc.1 -= 1,
            _ => (),
        }
    }
    loc_map.insert(loc);

    (loc_map.len(), bench.elapsed())
}
fn vec_impl(input: &str) -> (usize, Duration) {
    let bench = Instant::now();

    let mut loc_map: Vec<(i32, i32)> = vec![];
    let mut loc: (i32, i32) = (0, 0);

    for direction in input.bytes() {
        if ! loc_map.contains(&loc) {
            loc_map.push(loc);
        }
        match direction {
            b'>' => loc.0 += 1,
            b'<' => loc.0 -= 1,
            b'v' => loc.1 += 1,
            b'^' => loc.1 -= 1,
            _ => (),
        }
    }
    loc_map.push(loc);

    (loc_map.len(), bench.elapsed())
}
