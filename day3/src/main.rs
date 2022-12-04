use std::{io::{BufReader, BufRead}, fs::File};

fn main() {
    let matches = get_matches();
    let mut sum = 0;
    for m in matches {
        sum += get_match_priority(m);
    }
    println!("Total sum {}", sum);
}

fn get_matches() -> Vec<char>{
    let mut input = Vec::new();
    let file = File::open("input/actual.txt")
        .expect("File not found");
    let reader = BufReader::new(file);
    for l in reader.lines() {
        let line = l.expect("Can't parse line");
        let (compartment_a, compartment_b) = line.split_at(line.len()/2);
        let m_index = compartment_a.find(|a: char| compartment_b.find(a) != None).expect("No matches!");
        let char_vec: Vec<char> = compartment_a.chars().collect();
        let m = char_vec[m_index];
        println!("Split to {} and {}. Found a match {}", compartment_a, compartment_b, m);
        input.push(m);
    }
    return input;
}

fn get_match_priority(m: char) -> u32{
    let mut value = m.to_digit(36).expect("Invalid character");
    value -= 9;
    if m.is_uppercase(){
        value += 26;
    }
    println!("Char {} has value {}", m, value);
    return value;
}