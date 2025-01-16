use std::{env, fs};

fn part1(levels: &Vec<Vec<i64>>) -> i64 {
    let mut safe_count = 0;

    for report in levels.iter() {
        if is_safe(report) {
            safe_count += 1;
        }
    }

    safe_count
}

fn part2(levels: &mut Vec<Vec<i64>>) -> i64 {
    let mut safe_count = 0;

    for report in levels.iter_mut() {
        if is_safe(report) {
            safe_count += 1;
        } else if can_be_made_safe(report) {
            safe_count += 1;
        }
    }

    safe_count
}

fn is_safe(report: &Vec<i64>) -> bool {
    let mut direction = None;

    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if (1..=3).contains(&diff) {
            if direction == Some(false) {
                return false;
            }
            direction = Some(true);
        } else if (-3..=-1).contains(&diff) {
            if direction == Some(true) {
                return false; 
            }
            direction = Some(false);
        } else {
            return false;
        }
    }

    true
}

fn can_be_made_safe(report: &mut Vec<i64>) -> bool {
    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);

        if is_safe(&modified_report) {
            return true;
        }
    }

    false 
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have read file");

    let levels: Vec<Vec<i64>> = contents
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|val| val.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    println!("{:?}", contents);

    let part1_safe_count = part1(&levels);
    let part2_safe_count = part2(&mut levels.clone());

    println!("Safe reports: {}", part1_safe_count); //321 when running file.txt
    println!("Safe reports: {}", part2_safe_count); //386 when running file.txt
}
