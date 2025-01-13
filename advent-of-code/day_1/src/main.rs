use std::{env, fs};

fn calculate_difference(left: &[i64], right: &[i64]) -> i64 {
    let substracted: Vec<i64> = left.into_iter().zip(right).map(|(a,b)| (b - a).abs()).collect();
    substracted.into_iter().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have read file");
    let mut left_array: Vec<i64> = contents
        .split_whitespace()
        .enumerate()
        .filter(|&(idx,_)| idx % 2 == 0)
        .map(|(_, data)| data.parse::<i64>().unwrap())
        .collect();
    
    let mut right_array: Vec<i64> = contents
        .split_whitespace()
        .enumerate()
        .filter(|&(idx,_)| idx % 2 == 1)
        .map(|(_, data)| data.parse::<i64>().unwrap())
        .collect();

    left_array.sort(); 
    right_array.sort(); 

    let answer = calculate_difference(&left_array , &right_array);

    //println!("{contents}");
    println!("{:?}", answer);

}
