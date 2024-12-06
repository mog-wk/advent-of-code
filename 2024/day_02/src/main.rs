fn main() {
    let reports: Vec<_> = std::fs::read_to_string("./day_02/src/input.txt")
        .expect("wrong input path")
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|level| level.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    //println!("task 01: {}", task_01(&reports));
    println!("task 02: {}", task_02(&reports)); // test = 4
}
fn task_02(reports: &Vec<Vec<i32>>) -> u32 {
    let mut safe_reports = 0;
    for report in reports {
        // change is_safe to dictate damping level(None = report is safe(same as true in task one),
        // Some(i) = possible safe one error in report[i] or report[i + 1]
        let mut is_safe = None;
        // dictates the order of the report (true = ascending, false = descending, None = not
        // defined yet
        let mut ordering = None;
        for (i, level) in report.windows(2).enumerate() {
            // check ordering
            match ordering {
                Some(order) => {
                    if !((level[1] - level[0] > 0 && order) || (level[1] - level[0] < 0 && !order))
                    {
                        is_safe = Some(i);
                        break;
                    }
                }
                None => {
                    ordering = if level[1] - level[0] > 0 {
                        Some(true)
                    } else {
                        Some(false)
                    }
                }
            }
            let dif = (level[0] - level[1]).abs();
            // handle greater difference and no difference
            if dif > 3 || dif == 0 {
                is_safe = Some(i);
                break;
            }
        }
        if let Some(damp_idx) = is_safe {
            // removes damp element from report
            let damp_report = [&report[..damp_idx], &report[damp_idx + 1..]].concat();

            if is_report_safe(&damp_report) {
                //println!( "safe with left damp: {:?} if {} is removed from index: {}\t{:?}", report, report[damp_idx], damp_idx, damp_report);
                safe_reports += 1;
                continue;
            }
            // check for item to the right of the damp error
            let damp_report = [&report[..damp_idx + 1], &report[damp_idx + 2..]].concat();

            if is_report_safe(&damp_report) {
                println!(
                    "safe with right damp: {:?} if {} is removed from index: {}\t{:?}",
                    report,
                    report[damp_idx + 1],
                    damp_idx + 1,
                    damp_report,
                );
                safe_reports += 1;
                continue;
            }

            // check first index ordering edge case
            if is_report_safe(&report[1..].to_vec()) {
                safe_reports += 1;
                continue;
            }
            println!("unsafe: {:8>?} {:?}", report, damp_report);
        } else {
            //println!("safe without damp: {:8>?}", report);
            safe_reports += 1;
        }
    }
    safe_reports
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut ordering = None;
    // reassign true = safe, false = not safe
    let mut is_safe = true;

    for level in report.windows(2) {
        match ordering {
            Some(order) => {
                if !((level[1] - level[0] > 0 && order) || (level[1] - level[0] < 0 && !order)) {
                    is_safe = false;
                    break;
                }
            }
            None => {
                ordering = if level[1] - level[0] > 0 {
                    Some(true)
                } else {
                    Some(false)
                }
            }
        }
        let dif = (level[0] - level[1]).abs();
        // handle greater difference and no difference
        if dif > 3 || dif == 0 {
            is_safe = false;
            break;
        }
    }

    is_safe
}

fn task_01(reports: &Vec<Vec<i32>>) -> u32 {
    let mut safe_reports = 0;
    for report in reports {
        let mut is_safe = true;
        // dictates the order of the report (true = ascending, false = descending, None = not
        // defined yet
        let mut ordering = None;
        for level in report.windows(2) {
            let dif = (level[0] - level[1]).abs();
            match ordering {
                Some(order) => {
                    if !((level[1] - level[0] > 0 && order) || (level[1] - level[0] < 0 && !order))
                    {
                        is_safe = false;
                        break;
                    }
                }
                None => {
                    if level[1] - level[0] > 0 {
                        ordering = Some(true);
                    } else {
                        ordering = Some(false);
                    }
                }
            }
            // handle greater difference and no difference
            if dif > 3 || dif == 0 {
                is_safe = false;
                break;
            }
        }
        if is_safe {
            println!("{:?}", report);
            safe_reports += 1;
        }
    }
    safe_reports
}
