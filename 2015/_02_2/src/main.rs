/*
   --- Part Two --- The elves are also running low on ribbon. Ribbon is all the same width, so they
   only have to worry about the length they need to order, which they would again like to be exact.

   The ribbon required to wrap a present is the shortest distance around its sides, or the smallest
   perimeter of any one face. Each present also requires a bow made out of ribbon as well; the feet
   of ribbon required for the perfect bow is equal to the cubic feet of volume of the present.
   Don't ask how they tie the bow, though; they'll never tell.

   For example:

   A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the present plus
   2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet.
   A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the present plus
   1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.

   How many total feet of ribbon should they order?
*/

use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    File::open("assets/02.txt").unwrap().read_to_string(&mut input).unwrap();

    let begin_time = Instant::now();

    let areas: Vec<&str> = input.split('\n').collect();

    let mut total_ribbon = 0;
    for area in &areas[0..areas.len()-1] {
        let mut sides: Vec<u32> = area.split('x').map(|e| e.parse().unwrap()).collect();
        let sides = sort(&mut sides);

        total_ribbon += sides.0 * 2 + sides.1 * 2 + sides.0 * sides.1 * sides.2;
    }


    let elp = begin_time.elapsed();

    println!("{} | {:.2?}", total_ribbon, elp);
}

/// insertion sort adapted for convenience
fn sort(arr: &mut Vec<u32>) -> (u32, u32, u32) {
    for i in 1..3 {
        let mut j = i;
        while j > 0 && arr[j] < arr[j-1] {
            arr.swap(j, j-1);
            j-=1;
        }
    }
    (arr[0], arr[1], arr[2])
}
