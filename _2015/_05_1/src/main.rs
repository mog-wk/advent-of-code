/*--- Day 5: Doesn't He Have Intern-Elves For This? ---

Santa needs help figuring out which strings in his text file are naughty or nice.

A nice string is one with all of the following properties:

    It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
    It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
    It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.

For example:

    ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
    aaa is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
    jchzalrnumimnmhp is naughty because it has no double letter.
    haegwjzuvuyypxyu is naughty because it contains the string xy.
    dvszwmarrgswjxmb is naughty because it contains only one vowel.

How many strings are nice?
*/

use std::fs::File;
use std::io::Read;

use std::time::Instant;

use std::collections::HashSet;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    File::open("assets/05.txt").unwrap().read_to_string(&mut input).unwrap();

    let bench = Instant::now();

    let forbidden_strings: [&str; 4] = ["ab", "cd", "pq", "xy"];

    let string_vec: Vec<&str> = input.split('\n').collect();

    let mut nice_strings_counter: u32 = 0;
    //let mut nice_strings: HashSet<&str> = HashSet::new();

    //'run: for name in string_vec[0..10].iter() {
    'run: for name in string_vec[0..string_vec.len()-1].iter() {
        // skip if name has a forbidden string
        for sequence in forbidden_strings.iter() {
            if name.contains(sequence) {
                continue 'run;
            }
        }

        let mut vowels_set: HashSet<u8> = HashSet::new();
        let mut vowels_count = 0;
        let mut letter_buffer = b'_';
        let mut has_double_letter = false;

        for byte in name.bytes() {
            // check vowels
            let _inserted_vowel = match byte {
                b'a' | b'e' | b'i' | b'o' | b'u' => {
                    vowels_count += 1;
                    vowels_set.insert(byte)
                },
                _ => false,
            };
            //check doplicate letter
            if has_double_letter { continue }
            if byte == letter_buffer {
                has_double_letter = true;
            }
            letter_buffer = byte;
        }

        if (vowels_count >= 3) && (has_double_letter == true) {
            nice_strings_counter += 1;
            //nice_strings.insert(&name);
        }
        println!("{} {:?} {} {}", name, vowels_set, has_double_letter, nice_strings_counter);
    }

    println!("{} {:.2?}", nice_strings_counter, bench.elapsed());

}
