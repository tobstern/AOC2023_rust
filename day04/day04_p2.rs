use std::borrow::Borrow;
use std::fs;
use std::{cell::RefCell, collections::HashMap};

// #[derive(Debug)]
pub fn read_txt(day: String) -> String {
    // read in puzzle input: dayXX.txt
    let suffix: &str = ".txt";

    let file_path: String = "puzzle_inputs/day".to_owned() + &day + &suffix;

    let text: String = fs::read_to_string(file_path).expect("Could not open the text-file");

    return text;
}

fn has_won(curr_wins: Vec<&str>, wins: Vec<Vec<&str>>, mine: Vec<Vec<&str>>, curr: i32) -> i32 {
    // loop through count of inner vectors and check if the win numbers are in "whatihave"
    // loop through one set:
    let mut count: i32 = 0;

    for num_str in curr_wins.iter() {
        // eliminate "":
        if num_str == &"" {
            continue;
        }
        // check -> curr - 1, because Cards start at 1
        if mine[(curr - 1) as usize].contains(num_str) {
            // i have the winning number -> count up
            count += 1;
        } else {
            // no winning number in set
            continue;
        }
    }
    count
}

fn check_winning_cards(
    // won_num: i32,
    curr_card: i32,
    instances: HashMap<i32, i32>,
    wins: Vec<Vec<&str>>,
    mine: Vec<Vec<&str>>,
) -> () {
    //-> HashMap<i32, i32> {
    // check current card for wins:

    // open newly found instance card:
    let curr_wins: &Vec<&str> = &wins.clone()[(curr_card - 1) as usize];
    // let next_won_num: i32 = has_won(curr_wins.clone(), wins.clone(), mine.clone(), next_curr);

    let won_num: i32 = has_won(curr_wins.clone(), wins.clone(), mine.clone(), curr_card);

    // print debug:
    println!();
    println!("instances (in check_winning_cards()): {:?}", &instances);
    println!("current_card: {} - count of wins: {}", &curr_card, &won_num);

    // last won number
    if won_num < 1 {
        // break off recursion
        return; //&instances;
    }

    // let card_ids: Vec<i32> = Vec::new();
    // create new instances:
    for offset in 1..=won_num {
        // save card id
        let next_curr: i32 = curr_card + offset;

        let ret_val: i32 = match instances.get(&next_curr) {
            Some(x) => *x,
            None => 0,
        };

        instances.insert(next_curr, ret_val + 1); // unwrap if not none else give 0!

        check_winning_cards(
            //next_won_num,
            next_curr,
            instances,
            wins.clone(),
            mine.clone(),
        );
    }
    // consider maybe to return instances?
    // &instances
}

fn main() {
    // let day = String::from("04");
    let day = String::from("04_test");

    // read in the text-file
    let txt: String = read_txt(day);

    let lines = txt.split("\n");

    let line_vec: Vec<&str> = lines.collect();

    let mut cards: Vec<Vec<String>> = vec![];
    let mut wins_str: Vec<Vec<&str>> = vec![];
    let mut whatihave_str: Vec<Vec<&str>> = vec![];

    // the result sum
    let mut sum: i32 = 0;

    // parse the input:
    for l in &line_vec {
        let temp = l
            .clone()
            .split(": ")
            .map(|x: &str| {
                x.split(" | ")
                    // .collect::<Vec<String>>()
                    // .iter()
                    .map(|x: &str| x.split(" ").collect::<Vec<_>>())
            })
            .collect::<Vec<_>>();

        // println!("{:?}", &temp);
        for new_l in temp {
            // println!("new line: {:?}", new_l.clone().collect::<Vec<_>>());
            let temp_vec: Vec<_> = new_l.clone().collect();
            // println!("temp_vec: {:?}", &temp_vec[0]);

            if temp_vec[0].contains(&"Card") {
                continue;
            } else {
                // save each vec
                wins_str.push(temp_vec[0].clone());
                whatihave_str.push(temp_vec[1].clone());
            }
        }
    }
    //
    println!("wins_str: {:?}", &wins_str);
    println!("whatihave_str: {:?}", &wins_str);

    // recursive scratch cards:
    // number of winning nums -> "copies" next num cards -> go in each card again -> check winning cards and "copy"
    // -> until no cards are won!
    let mut instances: HashMap<i32, i32> = HashMap::new();
    let mut ind: usize = 0;

    let temp_inst: HashMap<i32, i32> = instances.clone();

    for (i, curr_wins) in wins_str.iter().enumerate() {
        // start counting from 1:
        ind = i + 1_usize;

        // loop through upper set:
        let win_cnt: i32 = has_won(
            curr_wins.clone(),
            wins_str.clone(),
            whatihave_str.clone(),
            ind as i32,
        );

        temp_inst = instances.clone();
        // check if returned value is None -> not yet set!
        let ret_val: i32 = match temp_inst.get(&(ind as i32)) {
            Some(x) => *x,
            None => 0,
        };

        instances.insert(ind as i32, ret_val + win_cnt);

        check_winning_cards(
            // win_cnt,
            ind as i32,
            instances,
            wins_str.clone(),
            whatihave_str.clone(),
        );
    }

    // result:
    println!("The result is: {}", instances.values().sum::<i32>());
}
