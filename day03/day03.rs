use std::collections::HashMap;
use std::fs;

// #[derive(Debug)]
pub fn read_txt(day: String) -> String {
    // read in puzzle input: dayXX.txt
    let suffix: &str = ".txt";

    let file_path: String = "puzzle_inputs/day".to_owned() + &day + &suffix;

    let text: String = fs::read_to_string(file_path).expect("Could not open the text-file");

    return text;
}

fn check_adj(
    (i, j): &(i32, i32),
    scheme: &HashMap<(i32, i32), char>,
    (max_i, max_j): (i32, i32),
) -> bool {
    // return true if a symbol is adjacent to current number
    let dirs: Vec<(i32, i32)> = Vec::from([
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ]);

    // let test_str: String = "0123456789.".to_string();

    let mut cont: bool = false;

    for (m, n) in dirs {
        // test adjacents -> update cont (bool)
        // here cannot be a number adjacent to current num

        // test if reached boundaries:
        if ((i + m) < 0) | ((j + n) < 0) | ((i + m) >= max_i) | ((j + n) >= max_j) {
            // if reached any boundary -> do not use it
            continue;
        }

        let neigh: &char = scheme.get(&(i + m, j + n)).unwrap();

        println!("neigh: '{}' at pos: {:?}", neigh, &(i + m, j + n));
        if (neigh != &'.') & !neigh.is_numeric() {
            // it must be a nice character!
            cont = true;
            break; // found it break for loop
        }
    }
    // return cont
    cont
}

fn main() {
    // let day = String::from("03_test");
    let day = String::from("03");

    // read in the text-file
    let txt: String = read_txt(day);

    let lines = txt.split("\n");

    let line_vec: Vec<&str> = lines.collect();

    let max_i: i32 = line_vec.len() as i32;
    let max_j: i32 = line_vec[0].len() as i32;

    // HashMap for all fields
    let mut schematic: HashMap<(i32, i32), char> = HashMap::new();

    // create schematic
    for i in 0..(max_i as usize) {
        // let mut temp: Vec<String> = Vec::new();
        for j in 0..(max_j as usize) {
            //
            // println!(
            //     "row: {}, (cols, chars): {:?}",
            //     i,
            //     (j, cols.chars().to_string())
            // );

            // save position, cols.split() in HasMap:
            schematic.insert((i as i32, j as i32), line_vec[i].chars().nth(j).unwrap());
        }
    }

    println!("{:?}", schematic);

    // now loop through every position and check if
    // "any number" is touching "any symbol" except "."!
    // then collect the whole number and sum it up!
    // (numbers can not touch each other, )
    // check if numeric - check adjacents (vector of directions?) -> if adjacents not contain (alltogether as String?) ".|0-9" -> append to whole_num_string -> check if digit is on right? -> sum the number!
    let mut sum: i32 = 0;

    let mut curr_num: i32 = 0;

    // whole nnumber string:
    let mut temp_num: String = "".to_string();

    // part_number boolean:
    let mut is_part_num: bool = false;

    // right character - tested if current char is numeric
    let mut right_ch: &char = &'.';

    for i in 0..(max_i as usize) {
        // let mut temp: Vec<String> = Vec::new();
        for j in 0..(max_j as usize) {
            //
            // println!("{:?}", schematic.get(&(i, j)).unwrap() == &'.');
            let ch: &char = schematic.get(&(i as i32, j as i32)).unwrap();

            // check if next char is numeric:
            if ch.is_numeric() {
                println!("\nfound digit char: {}", &ch);
                // found number

                if !is_part_num {
                    // only change if is_part_num was set again to false
                    // (done after saving the current number, if is_part_num is true)

                    is_part_num = check_adj(&(i as i32, j as i32), &schematic, (max_i, max_j));
                }

                // append number string to temporary number
                temp_num.push_str(&ch.to_string());

                if (j + 1) < (max_j as usize) {
                    right_ch = schematic.get(&(i as i32, (j as i32) + 1)).unwrap();

                    println!("right char: {}", &right_ch);
                } else {
                    // reached boundary:
                    right_ch = &'.'
                }

                if is_part_num & !right_ch.is_numeric() {
                    // this number must be saved -> is it complete yet?

                    // save the number!
                    println!("temporary number: {}", &temp_num);
                    sum += temp_num.to_string().parse::<i32>().unwrap();
                    println!("total sum: {}", &sum);

                    // clear the temp_num_string:
                    temp_num = "".to_string();

                    // reset is_part_num
                    is_part_num = false;
                } else if !is_part_num & !right_ch.is_numeric() {
                    // either way clear the num string
                    temp_num = "".to_string();
                }
            }
        }
    }

    println!("\nThe result is: {:?}", sum);
    // println!("{}", '.'.is_numeric());
}
