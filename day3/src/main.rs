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
    let mut all_lines = Vec::new();
    let file = File::open("input/actual.txt")
        .expect("File not found");
    let reader = BufReader::new(file);
    for l in reader.lines() {
        let line = l.expect("Can't parse line");
        all_lines.push(line);
    }
    let mut i = 0;
    let num_of_lines = all_lines.len();
    let mut line_iter = all_lines.into_iter();
    loop{
        if i+3 > num_of_lines{
            break;
        }
        let sack_a = line_iter.next().expect("Index over");
        let sack_b = line_iter.next().expect("Index over");
        let sack_c = line_iter.next().expect("Index over");

        println!("Sacks {}, {}, {}", sack_a, sack_b, sack_c);
        // let (compartment_a, compartment_b) = line.split_at(line.len()/2);
        let m_index = sack_a.find(|a: char| sack_b.find(a) != None && sack_c.find(a) != None).expect("No matches!");
        let char_vec: Vec<char> = sack_a.chars().collect();
        let m = char_vec[m_index];
        println!("Found a match {}", m);
        input.push(m);
        i+=3;
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