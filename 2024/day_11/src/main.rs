use std::{collections::HashMap, hash::BuildHasher, time::Instant};

fn main() {
    println!("Hello, world!");

    let input = std::fs::read_to_string("./input.txt").unwrap();
    //let input = [125, 17];

    let input: &Vec<u64> = &input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|elt| elt.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>()[0];

    let t = Instant::now();
    println!(
        "task 01: {:?} in {:}_ms",
        task_02(&input, 25),
        t.elapsed().as_millis()
    );
    println!(
        "task 02: {:?} in {:}_ms",
        task_02(&input, 75),
        t.elapsed().as_millis()
    );
}

// https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2024/day11.rs
fn task_02(input: &[u64], blinks: usize) -> usize {
    let mut stones = Vec::with_capacity(5000);
    let mut todo = Vec::new();

    let mut indeces = HashMap::with_capacity(5000);

    let mut current = Vec::new();

    for &number in input {
        if let Some(&index) = indeces.get(&number) {
            current[index] += 1;
        } else {
            indeces.insert(number, indeces.len());
            todo.push(number);
            current.push(1);
        }
    }

    for _ in 0..blinks {
        let numbers = todo;
        todo = Vec::with_capacity(200);

        let mut index_of = |number| {
            let size = indeces.len();
            *indeces.entry(number).or_insert_with(|| {
                todo.push(number);
                size
            })
        };
        for number in numbers {
            let (first, second) = if number == 0 {
                (index_of(1), usize::MAX)
            } else {
                let digits = number.ilog10() + 1;
                if digits & 1 == 1 {
                    (index_of(number * 2024), usize::MAX)
                } else {
                    let power = 10_u64.pow(digits / 2);
                    (index_of(number / power), index_of(number % power))
                }
            };
            stones.push((first, second));
        }

        let mut next = vec![0; indeces.len()];

        for (&(first, second), amount) in stones.iter().zip(current) {
            next[first] += amount;
            if second != usize::MAX {
                next[second] += amount;
            }
        }

        current = next;
    }

    current.iter().sum()
}
