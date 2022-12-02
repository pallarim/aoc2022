use std::{io::{BufReader, BufRead}, fs::File};

const PLAYER_A_ROCK: &str = "A";    
const PLAYER_A_PAPER: &str = "B";
const PLAYER_A_SCISSORS: &str = "C";
const PLAYER_B_ROCK: &str = "X";
const PLAYER_B_PAPER: &str = "Y";
const PLAYER_B_SCISSORS: &str = "Z";
const WIN_POINTS:i32 = 6;
const DRAW_POINTS:i32 = 3;
const PAPER_POINTS:i32 = 2;
const ROCK_POINTS:i32 = 1;
const SCISSORS_POINTS:i32 = 3;

fn main() {
    //let strategy = get_strategy();
    let input = get_input();


    let mut player_a_total_points = 0;
    let mut player_b_total_points = 0;
    for i in input{
        //let response = strategy.get_key_value(&i).expect("Invalid input");//get_response(&i).expect("Invalid input");
        let mut keypair = i.split_whitespace();
        let p1 = (*keypair.nth(0).expect("Impossible")).to_string();
        let p2 = (*keypair.nth(0).expect("Can happend")).to_string();
        let p2_hand = get_hand(&p1, &p2);
        let result = get_result(&p1, &p2_hand);
        player_a_total_points += get_hand_points(&p1);
        player_b_total_points += get_hand_points(&p2_hand);
        if result == 1 {
            // player a win
            player_a_total_points += WIN_POINTS;
        } else if result == -1 {
            // player b win
            player_b_total_points += WIN_POINTS;
        } else {
            // tie
            player_a_total_points += DRAW_POINTS;
            player_b_total_points += DRAW_POINTS;
        }
    }

    println!("Player a - Player b, {} - {}", player_a_total_points, player_b_total_points);
}

fn get_hand<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    return match p2{
        PLAYER_B_PAPER => match p1 {
            PLAYER_A_PAPER => PLAYER_B_PAPER,
            PLAYER_A_ROCK => PLAYER_B_ROCK,
            PLAYER_A_SCISSORS => PLAYER_B_SCISSORS,
            _ => panic!()
        },
        PLAYER_B_ROCK => match p1 {
            PLAYER_A_PAPER => PLAYER_B_ROCK,
            PLAYER_A_ROCK => PLAYER_B_SCISSORS,
            PLAYER_A_SCISSORS => PLAYER_B_PAPER,
            _ => panic!()
        },//lose
        PLAYER_B_SCISSORS => match p1 {
            PLAYER_A_PAPER => PLAYER_B_SCISSORS,
            PLAYER_A_ROCK => PLAYER_B_PAPER,
            PLAYER_A_SCISSORS => PLAYER_B_ROCK,
            _ => panic!()
        },//win
        _ => panic!()
    }
}

// fn get_strategy() -> HashMap<String, String>{
//     let mut strategy = HashMap::new();
//     let file = File::open("input/strategy.txt")
//         .expect("File not found");
//     let reader = BufReader::new(file);
//     for l in reader.lines() {
//         let line = l.expect("Can't parse line");
//         println!("Found line {}", line);
//         let mut keypair = line.split_whitespace();
//         let key = (*keypair.nth(0).expect("Impossible")).to_string();
//         let value = (*keypair.nth(0).expect("Can happend")).to_string();
        
//         println!("Parsed values {} & {}", key, value);
//         strategy.insert(key, value);
//     }
//     return strategy;
// }

fn get_input() -> Vec<String>{
    let mut input = Vec::new();
    let file = File::open("input/actual.txt")
        .expect("File not found");
    let reader = BufReader::new(file);
    for l in reader.lines() {
        let line = l.expect("Can't parse line");
        input.push(line);
    }
    return input;
}

fn get_result(player_a: &str, player_b: &str) -> i32{
    return match player_a{
        PLAYER_A_PAPER => 
             match player_b {
                PLAYER_B_ROCK =>  1,
                PLAYER_B_PAPER =>  0,
                PLAYER_B_SCISSORS =>  -1,
                _ => panic!()
            }
        ,
        PLAYER_A_ROCK => 
             match player_b {
                PLAYER_B_ROCK =>  0,
                PLAYER_B_PAPER =>  -1,
                PLAYER_B_SCISSORS =>  1,
                _ => panic!()
            }
        ,
        PLAYER_A_SCISSORS => 
             match player_b {
                PLAYER_B_ROCK =>  -1,
                PLAYER_B_PAPER =>  1,
                PLAYER_B_SCISSORS =>  0,
                _ => panic!()
            },
            _ => panic!()
        
    };
}

fn get_hand_points(hand: &str) -> i32{
    return match hand {
        PLAYER_A_PAPER| PLAYER_B_PAPER => PAPER_POINTS,
        PLAYER_A_ROCK| PLAYER_B_ROCK => ROCK_POINTS,
        PLAYER_A_SCISSORS| PLAYER_B_SCISSORS => SCISSORS_POINTS,
        _ => 0
    };
}