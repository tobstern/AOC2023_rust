//! --- Day 19: Aplenty ---
//! Description goes here!
use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;

pub fn parse(
    input: String,
) -> (
    HashMap<String, Vec<(String, String, i64, String)>>,
    Vec<HashMap<String, i64>>,
) {
    //-> Vec<HashMap<char, HashMap<char, (char, i64, String)>>, (char, i64)> {
    let lines: Vec<&str> = input.split("\n\n").collect();

    let split_lines: Vec<Vec<&str>> = lines
        .iter()
        .map(|str_part| str_part.split('\n').collect())
        .collect();
    // println!("{:?}", &workflows_str);

    let workflows = split_lines[0].clone();
    let ratings = split_lines[1].clone();
    let mut all_ratings = Vec::new();
    // println!("workflows: {:?}\nratings: {:?}", &workflows, &ratings);

    // parse ratings:
    let mut ratings_map = HashMap::new();
    for line in ratings.iter() {
        if line.len() < 1 {
            continue;
        }

        // trim, discard 1st and last ({}), and split at ',' and then at '='
        let temp: &str = &line[1..line.len() - 1];

        let temp_vec = temp.split(',').collect::<Vec<&str>>();

        for ele in temp_vec.iter() {
            let val: Vec<&str> = ele.split('=').collect();
            ratings_map.insert(val[0].to_string(), val[1].parse::<i64>().unwrap_or(0));
        }

        // created full map for one line in all ratings, add to Vec
        all_ratings.push(ratings_map.clone());
    }

    // println!("{:?}", &all_ratings);

    // parse workflows:

    let mut workflows_map = HashMap::<String, Vec<(String, String, i64, String)>>::new();
    for line in workflows.iter() {
        // split at '{' then 2nd at ',' then each at ':' and then match in 1st the pattern 'char' + '>' or '<' + multiple digit integer
        let parts: Vec<&str> = line.split('{').collect();

        let second_parts: Vec<&str> = parts[1].split(',').collect();

        let mut conditions = Vec::new();

        for part in second_parts.iter() {
            // split at ':'
            let pattern = Regex::new(r"([xmas])([<>])(\d+)").unwrap();

            let sub_parts: Vec<&str> = part.split(':').collect();
            // let mut condition = String::new().as_str();

            if part.contains(':') {
                let condition = sub_parts[0].trim().to_string();
                let chars = sub_parts[1].trim().to_string();

                conditions.push(if let Some(matches) = pattern.captures(&condition) {
                    (
                        matches.get(1).map_or("", |m| m.as_str()).to_string(),
                        matches.get(2).map_or("", |m| m.as_str()).to_string(),
                        matches
                            .get(3)
                            .map_or("", |m| m.as_str())
                            .parse::<i64>()
                            .unwrap_or(0),
                        chars.clone(),
                    )
                } else {
                    (String::from(""), String::from(""), 0, String::from(""))
                });
            } else {
                // it is one of: [A, R, [a-z]{2,}]

                if part.contains('}') {
                    let chars = part[..part.len() - 1].trim().to_string();
                    conditions.push((String::from(""), String::from(""), 0, chars.clone()));
                } else {
                    let chars = part.trim().to_string();
                    conditions.push((String::from(""), String::from(""), 0, chars.clone()));
                }
            }
            // println!("Chars: {}", &chars);

            // let sub_part_2 = sub_parts[1];
        }
        // println!("Conditions: {:?}", conditions);

        // get the first string as key
        workflows_map.insert(parts[0].to_string(), conditions);
    }

    // println!("{:?}", &workflows_map);

    return (workflows_map, all_ratings);
}

#[allow(unused)]
pub fn part1(input: String) {
    let mut workflows = HashMap::new();
    let mut ratings = Vec::new();

    (workflows, ratings) = parse(input);
    println!("{:?}", &workflows);
    println!("{:?}", &ratings);

    // start timer
    let now = Instant::now(); // mark time

    // find the ultimately accepted parts,
    let chs = vec!["x", "m", "a", "s"];
    let mut result: i64 = 0;
    for rating in ratings.iter() {
        // and for each number:

        // start with in and follow conditions,
        // until hitting an A-accepted -> save the sum of all xmas(part) numbers
        // or until hitting a R-rejected -> just break.
        let mut key_str = "in"; // start
        let mut res_char = "x";

        'check_rating: loop {
            // check 1st condition, if not then 2nd, if not, ....
            if key_str == "R" {
                break 'check_rating;
            } else if key_str == "A" {
                // save sum of xmas values
                result += rating.values().sum::<i64>();
                break 'check_rating;
            }

            for cond in workflows.get(key_str).unwrap() {
                let mut failed: bool = false;

                if cond.0 == "" {
                    if res_char == "R" {
                        break 'check_rating;
                    } else if res_char == "A" {
                        // save sum of xmas values
                        result += rating.values().sum::<i64>();
                        break 'check_rating;
                    } else {
                        key_str = &cond.3;
                        break; // for loop
                    }
                } else {
                    // there is a condition:
                    let num = rating.get(&cond.0).unwrap();

                    if cond.1 == "<" {
                        // test if greater cond.2 -> failed, check next
                        if num > &cond.2 {
                            failed = true;
                            continue; // for looop
                        } else {
                            // true: go to cond.3 in conditions
                            key_str = &cond.3;
                            break; // for loop
                        }
                    } else {
                        if num < &cond.2 {
                            // test if smaller cond.2 -> failed, check next
                            failed = true;
                            continue; // for looop
                        } else {
                            // true: go to cond.3 in conditions
                            key_str = &cond.3;
                            break; // for loop
                        }
                    }
                }
            }
        }
    }

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("\nPart1 result is: {:?}", result);
}

#[allow(unused)]
pub fn part2(input: String) {
    let lines = input.split("\n");

    let line_vec: Vec<&str> = lines.collect();

    // start timer
    let now = Instant::now(); // mark time

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // println!("\nPart2 result is: {:?}", sum);
}
