/*
   --- Day 2: I Was Told There Would Be No Math ---

   The elves are running low on wrapping paper, and so they need to submit an order for more. They
   have a list of the dimensions (length l, width w, and height h) of each present, and only want
   to order exactly as much as they need.

   Fortunately, every present is a box (a perfect right rectangular prism), which makes calculating
   the required wrapping paper for each gift a little easier: find the surface area of the box,
   which is 2*l*w + 2*w*h + 2*h*l. The elves also need a little extra paper for each present: the
   area of the smallest side.

   For example: A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of
   wrapping paper plus 6 square feet of slack, for a total of 58 square feet.  A present with
   dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square
   foot of slack, for a total of 43 square feet.

   All numbers in the elves' list are in feet. How many total square feet of wrapping paper should
   they order?
*/

use std::fs::File;
use std::io::Read;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    File::open("assets/02.txt").unwrap().read_to_string(&mut input).unwrap();

    let areas: Vec<_> = input.split('\n').collect();
    let mut total_area: u32 = 0;
    for area in &areas[0..areas.len()-1] {
        let sides = calc_sides(area).unwrap();

        total_area += (2*sides.0 + 2*sides.1 + 2*sides.2) + sides.3;

        println!("{}", area);
    }

    println!("answer: {}", total_area);

}

fn calc_sides(area_str: &str) -> Result<(u32, u32, u32, u32), std::num::ParseIntError> {
    let dimentions: Vec<_> = area_str.split('x').collect();
    let l: u32 = dimentions[0].parse()?;
    let w: u32 = dimentions[1].parse()?;
    let h: u32 = dimentions[2].parse()?;

    let m = min(&mut [l*w, w*h, h*l], 3);

    Ok( (l*w, w*h, h*l, m) )
}

fn min(arr: &mut [u32], size: usize) -> u32 {
    let mut min = arr[0];
    for i in 1..size {
        if arr[i] < min {
            min = arr[i];
        }
    }
    min
}


