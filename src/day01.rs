//! # Day 1: Trebuchet?!
//!
//! First and last digit of each line form the calibration value. Find the sum
//! of all calibration values.
//!
//! [puzzle site](https://adventofcode.com/2023/day/1)
/// this function matches the digit strings to the corresponding number
/// and returns the number as a String
/// it matches also the reversed digit strings
pub fn match_digit(string: &str) -> &str {
    // match the String to i32
    let digit_string: &str = match string {
        "one" | "eno" => "1",
        "two" | "owt" => "2",
        "three" | "eerht" => "3",
        "four" | "ruof" => "4",
        "five" | "evif" => "5",
        "six" | "xis" => "6",
        "seven" | "neves" => "7",
        "eight" | "thgie" => "8",
        "nine" | "enin" => "9",
        _ => "",
    };

    digit_string
}

// Define your functions for part 1 and part 2 of the problem
pub fn part1(input: String) {
    let lines = input.split("\n");

    let line_vec: Vec<&str> = lines.collect();

    // save all numbers and pick 1st and last
    let mut cal_vals: Vec<i32> = vec![];

    for line in line_vec {
        let mut temp: Vec<String> = Vec::new();

        for cha in line.chars() {
            // println!("{}", cha);

            if cha.is_numeric() {
                // save into cal_vals
                let temp_str = String::from(cha);
                temp.push(temp_str)
            }
        }

        // remove numbers in between:
        if (temp.len() as i32) < 1 {
            // skip this
            continue;
        } else if (temp.len() as i32) == 1 {
            // double the number

            let s1: String = String::from(&temp[0]);

            print!("{:?}\t---\t", &s1);

            let s2: &str = &String::from(&temp[0]);

            cal_vals.push((s1 + s2).parse().unwrap());
        } else {
            // save first and last
            let first: String = String::from(&temp[0]);
            let last: String = String::from(&temp[temp.len() - 1]);

            print!("first: {:?}, last: {:?}\n", &first, &last);

            cal_vals.push((first + &last).parse().unwrap());
        }

        // add found numbers to new line
        // println!("{:?}", &temp)
    }
    let sum: i32 = cal_vals.iter().sum();

    println!("\nThe result is: {:?}", sum);
}

pub fn part2(input: String) {
    let lines = input.split("\n");

    let line_vec: Vec<&str> = lines.collect();

    // digit strings:
    let all_ds: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // digit strings reversed:
    let all_ds_rev: Vec<&str> = vec![
        "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
    ];

    // save all numbers and pick 1st and last
    let mut cal_vals: Vec<i32> = vec![];

    for line in line_vec {
        let mut temp: Vec<String> = Vec::new();
        let mut temp_rev: Vec<String> = Vec::new();
        let mut all_chars: String = String::from("");

        // search from left!
        println!();
        println!("Looking from the left side!");
        println!("line: {:?}", line.chars());
        for cha in line.chars() {
            // println!("{}", cha);

            let temp_str: String = cha.to_string();
            if cha.is_numeric() {
                // save into cal_vals
                temp.push(String::from(&temp_str));

                all_chars = String::from("");
            }

            // push to different vec![] every run
            // and check if contains a digit as &str
            all_chars.push_str(&temp_str); // push borrowed to owned!

            // check if contains
            if (all_chars.len() as i32) > 2 {
                // println!("{:?}", &all_chars);
                // now it can be a digit:
                for digit_string in all_ds.iter() {
                    // println!("{:?}", &digit_string);

                    if all_chars.contains(digit_string) {
                        // found a digit: push to temp, and clear the String all_chars
                        // convert string number to 'integer string'
                        let digit: &str = match_digit(digit_string);

                        // save is save
                        if digit != "" {
                            temp.push(digit.to_string());
                            all_chars = String::from("");
                        }
                    }
                }
            }
        }

        // search from right! (needed because of overlapping words)
        // clear all_chars for new loop
        println!("Looking now, from right side!");
        all_chars = String::from("");

        println!(
            "reversed line: {:?}",
            line.chars().rev().collect::<String>().chars()
        );
        for cha in line.chars().rev().collect::<String>().chars() {
            // println!("{}", cha);

            let temp_str: String = cha.to_string();
            if cha.is_numeric() {
                // save into temp String all_chars
                temp_rev.push(String::from(&temp_str));

                all_chars = String::from("");
            }

            // push to different vec![] every run
            // and check if contains a digit as &str
            all_chars.push_str(&temp_str); // push borrowed to owned!

            // check if contains
            if (all_chars.len() as i32) > 2 {
                // println!("{:?}", &all_chars);
                // now it can be a digit:
                for digit_string in all_ds_rev.iter() {
                    // println!("{:?}", &digit_string);

                    if all_chars.contains(digit_string) {
                        // found a digit: push to temp, and clear the String all_chars
                        // convert string number to 'integer string'
                        let digit: &str = match_digit(digit_string);

                        // save is save
                        if digit != "" {
                            temp_rev.push(digit.to_string());
                            all_chars = String::from("");
                        }
                    }
                }
            }
        }

        // save first and last
        let first: String = String::from(&temp[0]);
        let last: String = String::from(&temp_rev[0]);

        // print!("first: {:?}, last: {:?}\n", &first, &last);

        cal_vals.push((first + &last).parse::<i32>().unwrap());

        // add found numbers to new line
        // println!("{:?}", &temp)
    }
    // println!("calibration values: {:?}", cal_vals);
    let sum: i32 = cal_vals.iter().sum();

    println!("\nThe result is: {:?}", sum);
}
