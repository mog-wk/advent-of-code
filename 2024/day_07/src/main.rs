use std::ops::Div;

fn main() {
    let input = std::fs::read_to_string("./day_07/src/input.txt").unwrap();
    let input = std::fs::read_to_string("./day_07/src/tests/default_01.txt").unwrap();

    //println!("task 01: {}", task_01(&input));
    println!("task 02: {}", task_02(&input));
}

// calc concatenations
// TODO:
fn task_02(input: &str) -> u64 {
    let (aws, nums) = input
        .lines()
        .map(|line| {
            let (aws, nums) = line.split_once(':').expect("dead line");
            (
                aws.parse::<u64>().unwrap(),
                nums.trim()
                    .split(' ')
                    .into_iter()
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect(),
            )
        })
        .collect::<(Vec<u64>, Vec<Vec<u64>>)>();
    let ln = aws.len();

    let mut total_calibration = 0_u64;

    for i in 0..ln {
        let aws = aws[i];
        let nums = &nums[i];
        println!("{:>8?}: {:?}", aws, nums);

        if nums.len() == 1 {
            if aws == nums[0] {
                total_calibration += aws;
            }
            continue;
        }
        let mut tree = Vec::with_capacity(nums.len());

        tree.push(nums[0]);

        let mut prev_tree_level = 0;
        let mut done = false;

        for j in 1..nums.len() {
            let current_tree_level = 3_usize.pow(j as u32) + prev_tree_level;

            for k in prev_tree_level..current_tree_level {
                let parent = k.div(3);
                match k % 3 {
                    0 => {
                        // + operator
                        let val = tree[parent] + nums[j];
                        if val == aws {
                            println!("added: {:?} with val: {} from add", nums, val);
                            done = true;
                            break;
                        } else {
                            tree.push(val);
                        }
                    }
                    1 => {
                        // * operator
                        let val = tree[parent] * nums[j];
                        if val == aws {
                            println!("added: {:?} with val: {} from multiplication", nums, val);
                            done = true;
                            break;
                        } else {
                            tree.push(val);
                        }
                    }
                    2 => {
                        // || operator
                        let val = format!("{}{}", tree[parent], nums[j])
                            .parse::<u64>()
                            .unwrap();
                        if val == aws {
                            println!("added: {:?} with val: {} from concatenation", nums, val);
                            println!("Tree: {:#?} {}", tree, parent);
                            done = true;
                            break;
                        } else {
                            tree.push(val);
                        }
                    }
                    _ => unreachable!(),
                }
                //println!("{} | {} => {:?} {}", k, k % 3, tree, parent);
            }
            if done {
                total_calibration += aws;
                break;
            }
            prev_tree_level = current_tree_level;
        }
    }

    total_calibration
}

fn task_01(input: &str) -> u64 {
    let (aws, nums) = input
        .lines()
        .map(|line| {
            let (aws, nums) = line.split_once(':').expect("dead line");
            (
                aws.parse::<u64>().unwrap(),
                nums.trim()
                    .split(' ')
                    .into_iter()
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect(),
            )
        })
        .collect::<(Vec<u64>, Vec<Vec<u64>>)>();
    let ln = aws.len();
    //let ln = 10;

    let mut total_calibration = 0_u64;

    for i in 0..ln {
        let aws = aws[i];
        let nums = &nums[i];
        println!("{:>8?}: {:?}", aws, nums);

        if nums.len() == 1 {
            if aws == nums[0] {
                total_calibration += aws;
            }
            continue;
        }
        let mut tree = Vec::with_capacity(nums.len());

        tree.push(nums[0]);

        let mut prev_tree_level = 0;
        let mut done = false;

        for j in 1..nums.len() {
            let current_tree_level = 2_usize.pow(j as u32) + prev_tree_level;
            //println!("{:?} {:?}", prev_tree_level, current_tree_level);

            for k in prev_tree_level..current_tree_level {
                let parent = k.div(2);
                if k & 1 == 1 {
                    let val = nums[j] * tree[parent];
                    if val == aws {
                        println!("added: {:?} with val: {}", nums, val);
                        done = true;
                        break;
                    } else {
                        tree.push(val);
                    }
                } else {
                    let val = tree[parent] + nums[j];
                    if val == aws {
                        println!("added: {:?} with val: {}", nums, val);
                        done = true;
                        break;
                    } else {
                        tree.push(val);
                    }
                }
                //println!("{} => {:?} {} | {:?} {}", k, nums, j, tree, parent);
            }
            if done {
                total_calibration += aws;
                break;
            }
            prev_tree_level = current_tree_level;
        }
    }

    total_calibration
}
