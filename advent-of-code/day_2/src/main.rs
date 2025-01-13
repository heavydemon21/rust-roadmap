use std::{env, fs};

fn calculate_difference(left: &[i64], right: &[i64]) -> i64 {
    let mut answer = 0;
    for la in left {
        let mut counter = 0;
        for ra  in right {
            if la == ra {
               counter += 1; 
            }
        }
        answer += counter * la;
    }
    answer
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have read file");

    let left_array: Vec<i64> = contents
        .split_whitespace()
        .enumerate()
        .filter(|&(idx,_)| idx % 2 == 0)
        .map(|(_, data)| data.parse::<i64>().unwrap())
        .collect();
    
    let right_array: Vec<i64> = contents
        .split_whitespace()
        .enumerate()
        .filter(|&(idx,_)| idx % 2 == 1)
        .map(|(_, data)| data.parse::<i64>().unwrap())
        .collect();

    let answer = calculate_difference(&left_array , &right_array);

    //println!("{contents}");
    println!("{:?}", answer);
    // answer from file.txt = 18934359
}
