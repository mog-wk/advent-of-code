use std::fs;
use std::mem;

// TODO refactor clone calls

fn main() {
    println!("Hello, world!");
    let input = fs::read_to_string("assets/13.txt")
        .unwrap();
    let input = input[0..input.len()-1].split('\n')
        .collect::<Vec<&str>>();

    // number of peple present
    const ROW_LENGTH: u32 = 7;

    // matrix for storing sitting scores 
    // [None, Some(-57), Some(-62), Some(-75), Some(71), Some(-22), Some(-23), Some(-76)]
    // [Some(-14), None, Some(48), Some(89), Some(86), Some(-2), Some(27), Some(19)]
    // [Some(37), Some(45), None, Some(24), Some(5), Some(-68), Some(-25), Some(30)]

    let sitting_matrix = parse_data(input, ROW_LENGTH);


    let mut sum = 0;

    let mut consumed = [false; 1+ROW_LENGTH as usize];

    for (idx, row) in sitting_matrix.clone().into_iter().enumerate() {
        let mut cur_max: (usize, i32) = (0, -1000);
        for (i, e) in row.iter().enumerate() {
            match e {
                Some(n) => {
                    sum += n;
                    if *n > cur_max.1 {
                        cur_max.0 = i;
                        cur_max.1 = *n;
                    }
                },
                None => (),
            }
        }

        sum += cur_max.1;
        sum += sitting_matrix[cur_max.0][idx].unwrap();
        consumed[cur_max.0] = true;
        consumed[idx] = true;
        println!("{:?} {}", row.iter().map(|e| e.unwrap_or(0)).collect::<Vec<i32>>(), sum);

    }
}

fn parse_data(data: Vec<&str>, row_length: u32) -> Vec<Vec<Option<i32>>> {
    let mut sitting_matrix: Vec<Vec<Option<i32>>> = Vec::new();

    let mut present_row: u32 = 0;
    let mut row_counter: u32 = 0;

    let mut row: Vec<Option<i32>> = Vec::new();

    //for line in input[0..=ROW_LENGTH as usize * 7].iter() {
    for line in data.into_iter() {
        let line = line.split(' ').collect::<Vec<&str>>();

        let (user, increment, amount, target) = (line[0], line[2], line[3], line[10]);
        //println!("{} {} {} {}", user, increment, amount, target);

        mem::drop(line);

        if row_counter == present_row {
            row_counter += 1;
            row.push(None);
        }

        match increment {
            "lose" => row.push(Some(amount.parse::<i32>().unwrap() * -1)),
            "gain" => row.push(Some(amount.parse::<i32>().unwrap())),
            _ => panic!("unintended value"),
        }

        row_counter += 1;
        if row_counter > row_length {
            present_row += 1;
            row_counter = 0;

            // add and reset row
            sitting_matrix.push(row.clone());
            row = Vec::new();
        }
        if row_counter == row_length && present_row == row_length {
            present_row += 1;
            row_counter = 0;

            row.push(None);
            // add and reset row
            sitting_matrix.push(row.clone());
            row = Vec::new();
        }
    }
    sitting_matrix
}
