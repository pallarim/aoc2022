use std::{fs::{File}, io::{BufReader, BufRead}};
fn main() {
    let highest_sums = get_highest_sum(3);
    let total_sum:i32 = highest_sums.iter().sum();
    let hightst_sums_str: Vec<String> = highest_sums.into_iter().map(|i| i.to_string()).collect();
    let sums = hightst_sums_str.join(", ");
    println!("highest sums= {sums}\n");
    println!("total sum= {total_sum}\n");
}

fn get_highest_sum(result_count: usize) -> Vec<i32>{
    let file = File::open("input/actual.txt")
        .expect("File not found");
    let reader = BufReader::new(file);
    
    let mut list = Vec::new();
    let mut latest_sum = 0;
    let mut highest_sum = 0;
    for l in reader.lines() {
        let line = l.expect("Can't parse line");
        if line.is_empty() {
            println!("group collected value = {latest_sum}\n");
            list.push(latest_sum);
            latest_sum = 0;
        }else{
            let my_int = line.parse::<i32>().unwrap();
            latest_sum += my_int;
            if latest_sum > highest_sum{
                highest_sum = latest_sum;
            }
        }
    }
    println!("latest sum collected = {latest_sum}\n");
    list.push(latest_sum);

    list.sort();
    list.reverse();
    let (left, _right) = list.split_at(result_count);
    return left.to_vec();
}