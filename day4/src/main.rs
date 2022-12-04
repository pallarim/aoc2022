use std::{io::{BufReader, BufRead}, fs::File};

fn main() {
    let sections = get_sections();
    let mut fully_contains_count = 0;
    for i in 0..sections.len(){
        //check if fully contained
        let ai = &sections[i][0];
        let bi = &sections[i][1];
        if ai.iter().any(|n| bi.contains(n)) || bi.iter().any(|n| ai.contains(n)){
            fully_contains_count +=1;
            println!("Found a pair with i {}. Values {:?} and {:?}", i, ai, bi);
        }
            
    }
    println!("Found fully contained {}", fully_contains_count);
}

fn get_sections() -> Vec<Vec<Vec<i32>>>{
    let mut input = Vec::new();
    let file = File::open("input/actual.txt")
        .expect("File not found");
    let reader = BufReader::new(file);
    for l in reader.lines() {
        let line = l.expect("Can't parse line");
        let (a, b) = line.split_once(',').expect("WTF split");
        let a_values = get_range(a);
        let b_values = get_range(b);
        let mut pair = Vec::new();
        pair.push(a_values);
        pair.push(b_values);
        input.push(pair);
    }
    return input;
}

fn get_range(line: &str) -> Vec<i32>{
    let (from_a, to_a) = line.split_once('-').expect("WTF split");
    let mut a_values = Vec::new();
    for i in from_a.parse::<i32>().unwrap()..to_a.parse::<i32>().unwrap()+1{
        a_values.push(i);
    }
    return a_values;
}