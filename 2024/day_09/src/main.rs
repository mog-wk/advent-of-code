use std::{
    collections::{HashMap, HashSet},
    time::Instant,
    u8,
};

fn main() {
    let mut f = std::env::args();
    f.next();
    let f = f.next().unwrap_or("./day_09/src/input.txt".to_string());
    let input = std::fs::read_to_string(f).unwrap();

    let disk_map: Vec<u8> = input[..input.len() - 1].bytes().map(|b| b - 48).collect();

    //let _ = Instant::now();
    //println!("task 01: {}", task_01(&disk_map));

    let t = Instant::now();
    println!(
        "task 02: {} in {}_ns",
        task_02(&disk_map),
        t.elapsed().as_nanos()
    );
}

// compress without fragmentation
fn task_02(disk_map: &Vec<u8>) -> u64 {
    let (last_file_size, last_free_size) = if disk_map.len() & 1 == 1 {
        (*disk_map.last().unwrap(), 0_u8)
    } else {
        let i = *disk_map.last().unwrap();
        let j = *disk_map.last().unwrap();
        (j, i)
    };

    let mut disk_map: Vec<(u8, u8)> = disk_map
        .windows(2)
        .step_by(2)
        .map(|elt| (elt[0], elt[1]))
        .collect();
    disk_map.push((last_file_size, last_free_size));

    println!("{:?}", disk_map);

    let ln = disk_map.len();

    let mut translocated_ids = HashSet::new();

    let mut compressed_map: Vec<u8> = Vec::new();

    for i in 0..ln {
        let (mut file_size, mut free_size) = (disk_map[i].0, disk_map[i].1);
        println!("== id: {i} file_size: {file_size} free_size: {free_size}");

        if !translocated_ids.contains(&i) {
            while file_size > 0 {
                compressed_map.push(i as u8);
                translocated_ids.insert(i);
                println!(">> added foward => {compressed_map:?}");
                file_size -= 1;
            }
        }
        if free_size > 0 {
            for j in i + 1..ln {
                let j = ln - j;

                let checked_file_size = disk_map[j].0;
                println!("> checking {:?} {}", disk_map[j], j);

                if checked_file_size <= free_size {
                    if translocated_ids.insert(j) {
                        for _ in 0..checked_file_size {
                            compressed_map.push(j as u8);
                        }

                        disk_map[j].1 += disk_map[j].0;
                        disk_map[i].1 -= disk_map[j].0;
                        disk_map[j].0 = 0;

                        free_size -= checked_file_size;
                        println!(">> added backwards => {compressed_map:?}");
                    }
                }
            }
        }
        println!("<< {disk_map:?} {compressed_map:?} {translocated_ids:?}");
    }

    println!("<<<< {disk_map:?} {compressed_map:?}");
    12
}

#[allow(unused)]
fn works_but_defrags_diferently(disk_map: &Vec<u8>) -> u64 {
    let ln = disk_map.len();

    let mut compressed_map = Vec::new();

    let last_file_loc = if ln & 1 == 1 { ln - 1 } else { ln - 2 };

    let mut id_blacklist: HashSet<usize> = HashSet::new();

    for (id, i) in (0..last_file_loc).step_by(2).enumerate() {
        let (mut file_space, mut free_space): (_, u8) = if !id_blacklist.contains(&id) {
            (disk_map[i], *disk_map.get(i + 1).unwrap_or(&0))
        } else {
            (0, disk_map[i] + *disk_map.get(i + 1).unwrap_or(&0))
        };
        println!("{id} => {:?} {:?}", file_space, free_space);

        // add file into compressed map
        while file_space > 0 {
            println!("-> {:?} {}", id, file_space);
            compressed_map.push(id);
            println!("- added cur_id: {:?}", id);
            file_space -= 1;
        }

        // add files chunks if any
        for j in (0..=last_file_loc - i).step_by(2) {
            let checked_file_space = disk_map[ln - 1 - j];

            let checked_id = (ln - j) / 2;
            println!(
                "{} ==> checking: {}; size: {}; free_space: {}",
                ln - j - 1,
                checked_id,
                checked_file_space,
                free_space
            );
            if !id_blacklist.contains(&checked_id) && free_space >= checked_file_space {
                id_blacklist.insert(checked_id);
                while free_space >= checked_file_space {
                    for _ in 0..checked_file_space {
                        println!("added in checked {:?}", checked_id);
                        compressed_map.push(checked_id);
                        free_space -= 1;
                    }
                }
                break;
            }
        }
    }

    println!("{:?}", disk_map);
    println!("{:?}", compressed_map);
    get_checksum(&compressed_map)
}
// compress fs with fragmentation
fn task_01(disk_map: &Vec<u8>) -> u64 {
    let ln = disk_map.len();

    let (mut last_id, mut last_ptr) = if ln & 1 == 1 {
        (ln / 2, ln - 1)
    } else {
        ((ln - 1) / 2, ln - 2)
    };

    println!("{} {}", last_id, last_ptr);
    let mut last_amount = disk_map.get(last_ptr).unwrap().clone();

    let mut compressed_map = Vec::new();

    for (id, i) in (0..ln).step_by(2).enumerate() {
        let mut file_space = disk_map[i];
        let mut free_space: u8 = *disk_map.get(i + 1).unwrap_or(&0);
        println!("{:?} {:?}", file_space, free_space);

        if last_id <= id {
            // add remaining ids
            while last_amount > 0 {
                println!("& added bulk_amout: {:?}", last_id);
                compressed_map.push(last_id);
                last_amount -= 1;
            }
            break;
        }
        while file_space > 0 {
            println!("-> {:?} {}", id, file_space);
            compressed_map.push(id);
            println!("- added cur_id: {:?}", id);
            file_space -= 1;
        }

        while free_space > 0 {
            println!("=> {:?} {}", id, free_space);
            if last_amount > 0 {
                println!("- added last_id: {:?}", last_id);
                compressed_map.push(last_id);
                last_amount -= 1;
                free_space -= 1;
            } else {
                // update last ptr
                if last_id - 1 > id {
                    last_id -= 1;
                    last_ptr -= 2;
                    last_amount = *disk_map.get(last_ptr).unwrap();
                } else {
                    println!("=== last index cross break ===");
                    break;
                }
            }
        }
    }

    //println!("{:?}", disk_map);
    //println!("{:?}", compressed_map);
    get_checksum(&compressed_map)
}

fn get_checksum(map: &Vec<usize>) -> u64 {
    let mut sum: u64 = 0;
    for (i, id) in map.iter().enumerate() {
        sum += (*id as u64) * (i as u64);
    }
    sum
}
