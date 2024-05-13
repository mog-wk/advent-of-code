/*--- Part Two ---

Realizing the error of his ways, Santa has switched to a better model of determining whether a string is naughty or nice. None of the old rules apply, as they are all clearly ridiculous.

Now, a nice string is one with all of the following properties:

    It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
    It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.

For example:

    qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).
    xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
    uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.
    ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.

How many strings are nice under these new rules?
*/

use std::fs::File;
use std::io::Read;

use std::time::Instant;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    File::open("assets/05.txt").unwrap().read_to_string(&mut input).unwrap();

    let bench = Instant::now();

    let string_vec: Vec<&str> = input.split('\n').collect();

    let mut nice_strings_counter: u32 = 0;


    //for name in string_vec[0..10].iter() {
    for name in string_vec[0..string_vec.len()-1].iter() {
        let mut letter_buffer_prev = name[0..1].bytes().collect::<Vec<u8>>()[0];
        let mut letter_buffer_mid  = name[1..2].bytes().collect::<Vec<u8>>()[0];

        let mut has_letter_ : bool = false;

        // check xyx sequence
        for byte in name[2..].bytes() {
            //print!("{} {} | ", letter_buffer_mid, letter_buffer_prev);

            if has_letter_ { continue }
            if byte == letter_buffer_prev {
                has_letter_ = true;
            } else {
                letter_buffer_prev = letter_buffer_mid;
                letter_buffer_mid = byte;
            }
        }

        println!("{} {}", name, has_letter_);

        // skip if didn t found sequence
        if !has_letter_ { continue }

        let mut has_pair: bool = false;

        // check pairs TODO fix stuff
        let mut pairs: [((u8, u8), (usize, usize)); 15] = [((0,0), (1, 1)); 15];
        let mut i: usize = 1;
        let bytes = name[i..].bytes(); 
        let mut anchor: u8 = name[0..1].bytes().collect::<Vec<u8>>()[0];
        for byte in bytes {
            pairs[i-1] = ((anchor, byte), (i-1, i));
            i += 1;
            anchor = byte;
        }

        for pair_buffer in pairs[..pairs.len()-2].iter() {
            for pair in pairs[2..].iter() {
                if (pair.0 == pair_buffer.0) &&
                    !( pair.1.0 == pair_buffer.1.1 
                    || pair.1.1 == pair_buffer.1.0 
                    || pair.1.1 == pair_buffer.1.1 
                    || pair.1.0 == pair_buffer.1.0) { 
                    has_pair = true;
                    break;
                }
            }

            if has_pair {
                nice_strings_counter += 1;
                break
            }
        }

    }

    println!("{} {:.2?}", nice_strings_counter, bench.elapsed());

}
