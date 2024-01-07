//! --- Day 12: Hot Springs ---
//!
//! So, condition records with no unknown spring conditions might look like this:
//!
//! #.#.### 1,1,3
//! .#...#....###. 1,1,3
//! .#.###.#.###### 1,3,1,6
//! ####.#...#... 4,1,1
//! #....######..#####. 1,6,5
//! .###.##....# 3,2,1
//!
//! However, the condition records are partially damaged; some of the springs' conditions are actually unknown (?). For example:
//!
//! ???.### 1,1,3
//! .??..??...?##. 1,1,3
//! ?#?#?#?#?#?#?#? 1,3,1,6
//! ????.#...#... 4,1,1
//! ????.######..#####. 1,6,5
//! ?###???????? 3,2,1
//!
//! Equipped with this information, it is your job to figure out how many different arrangements of operational and broken springs fit the given criteria in each row.
//!
//! In the first line (???.### 1,1,3), there is exactly one way separate groups of one, one, and three broken springs (in that order) can appear in that row: the first three unknown springs must be broken, then operational, then broken (#.#), making the whole row #.#.###.
//!
//! The second line is more interesting: .??..??...?##. 1,1,3 could be a total of four different arrangements. The last ? must always be broken (to satisfy the final contiguous group of three broken springs), and each ?? must hide exactly one of the two broken springs. (Neither ?? could be both broken springs or they would form a single contiguous group of two; if that were true, the numbers afterward would have been 2,3 instead.) Since each ?? can either be #. or .#, there are four possible arrangements of springs.
//!
//! The last line is actually consistent with ten different arrangements! Because the first number is 3, the first and second ? must both be . (if either were #, the first number would have to be 4 or higher). However, the remaining run of unknown spring conditions have many different ways they could hold groups of two and one broken springs:
//!
//! ?###???????? 3,2,1
//! .###.##.#...
//! .###.##..#..
//! .###.##...#.
//! .###.##....#
//! .###..##.#..
//! .###..##..#.
//! .###..##...#
//! .###...##.#.
//! .###...##..#
//! .###....##.#
//!
//! In this example, the number of possible arrangements for each row is:
//!
//!     ???.### 1,1,3 - 1 arrangement
//!     .??..??...?##. 1,1,3 - 4 arrangements
//!     ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
//!     ????.#...#... 4,1,1 - 1 arrangement
//!     ????.######..#####. 1,6,5 - 4 arrangements
//!     ?###???????? 3,2,1 - 10 arrangements
//!
//! Adding all of the possible arrangement counts together produces a total of 21 arrangements.
//!
//! For each row, count all of the different arrangements of operational and broken springs that meet the given criteria. What is the sum of those counts?

// use indicatif::{ProgressBar, ProgressStyle};
// use rayon::prelude::*;
use std::{collections::HashMap, time::Instant};

#[allow(unused)]
pub fn generate_combinations(s: &mut Vec<char>, i: usize) -> Vec<Vec<char>> {
    if i == s.len() {
        return vec![s.clone()];
    }

    if s[i] == '?' {
        s[i] = '#';
        let mut result1 = generate_combinations(s, i + 1);
        s[i] = '.';
        let mut result2 = generate_combinations(s, i + 1);
        s[i] = '?';

        result1.append(&mut result2);
        return result1;
    } else {
        return generate_combinations(s, i + 1);
    }
}

#[allow(unused)]
pub fn part1(input: String) {
    let mut conditions: Vec<Vec<char>> = Vec::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for line in input.lines() {
        // println!("{}", line);
        let mut parts = line.split(" ");
        conditions.push(parts.next().unwrap().chars().collect::<Vec<char>>());
        groups.push(
            parts
                .next()
                .unwrap()
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        );
    }
    // println!("{:?}", conditions);
    // println!("{:?}", groups);

    // start timer
    let now = Instant::now(); // mark time

    // Find all possible combinations of the '#' groups
    // Each '#' in the condition is a group of 1, when not having neighbours of '#'
    // Each '?' can be replaced by either '#' or '.' and the groups can be combined
    // Any of the three symbol types can only be combined with neighbours of the same symbol type, and it will be a new group
    // The groups can only be combined in the order given in the groups list
    // The group numbers in the groups list are the sizes of the groups, and they give the size of groups of '#' only, in conditions list

    let mut count: usize = 0;
    let mut valid_combinations: usize = 0;

    for mut s_vec in conditions {
        let combinations = generate_combinations(&mut s_vec, 0);
        // println!("{:?}", combinations);
        for combination in combinations {
            let mut groups_vec: Vec<usize> = Vec::new();
            let mut group_size: usize = 0;
            for c in combination {
                if c == '#' {
                    group_size += 1;
                } else if group_size > 0 {
                    groups_vec.push(group_size);
                    group_size = 0;
                }
            }
            if group_size > 0 {
                groups_vec.push(group_size);
            }
            // println!("{:?}", groups_vec);
            if groups_vec == groups[count] {
                valid_combinations += 1;
                // println!("Found a match!");
                // break;
            }
        }
        count += 1;
    }

    println!("Total valid combinations: {}", valid_combinations);

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // println!("\nThe result is: {:?}", sum);
}

fn count(
    config_str: &str,
    nums: &[usize],
    mut cache: &mut HashMap<(String, Vec<usize>), usize>,
) -> usize {
    if config_str == "" {
        if nums.len() == 0 {
            // no more configurations (groups) to be - so valid grouping
            return 1;
        } else {
            // expecting more configurations (groups) to be - so invalid grouping
            return 0;
        }
    }

    if nums.len() == 0 {
        // no groups expected
        if config_str.contains('#') {
            // but still a '#' in the config_str - invalid
            return 0;
        } else {
            // no more groups expected and no '#' in the config_str - valid
            return 1;
        }
    }

    let key = (config_str.to_string(), nums.to_vec());
    if cache.contains_key(&key) {
        return cache[&key];
    }

    let mut result: usize = 0;

    if config_str.starts_with('?') || config_str.starts_with('.') {
        result += count(&config_str[1..], &nums, &mut cache);
    }

    let config_chars: Vec<char> = config_str.chars().collect();
    if config_str.starts_with('?') || config_str.starts_with('#') {
        // this is now a block
        // check if valid
        if nums[0] <= config_chars.len()
            && !config_chars[0..nums[0]]
                .iter()
                .collect::<String>()
                .contains(".")
            && (nums[0] == config_chars.len() || &config_chars[nums[0]] != &'#')
        {
            // valid, because we got the size we are expecting/ can expect
            // +1 because there must be a gap between blocks, even when it would be a '?'
            if nums[0] + 1 < config_chars.len() {
                result += count(
                    &config_chars[(nums[0] + 1)..].iter().collect::<String>(),
                    &nums[1..],
                    &mut cache,
                );
            } else {
                result += count("", &nums[1..], &mut cache);
            }
        }
    }

    // save key and result pair in cache
    cache.insert(key, result);

    result
}

#[allow(unused)]
pub fn part1_2nd_sol(input: String) {
    //
    // start timer
    let now = Instant::now(); // mark time

    // part 2: now each condition vec of all vecs needs to be 5 times itself, separated by '?'
    // and the groups vecs need to be 5 times itself too, the rest of the calculation is the same

    let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
    let mut conditions = Vec::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for line in input.lines() {
        // println!("{}", line);
        let mut parts = line.split(" ");
        // adapt to replicate conditions vec and groups vec 5 times each
        let mut temp_vec: Vec<char> = Vec::new();
        //let mut single_vec = parts.next().unwrap().chars().collect::<Vec<char>>();
        let mut single_vec = parts.next().unwrap();
        //for _ in 0..5 {
        // temp_vec.extend(single_vec.clone());
        //temp_vec.push('?');
        //}
        // remove last '?'
        // temp_vec.pop();
        // conditions.push(temp_vec);
        conditions.push(single_vec);

        let mut temp_vec: Vec<usize> = Vec::new();
        let mut single_vec = parts
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        //for _ in 0..5 {
        temp_vec.extend(single_vec.clone());
        //}
        groups.push(temp_vec);
    }
    // println!("{:?}", conditions);
    // println!("{:?}", groups);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // start timer
    let now = Instant::now(); // mark time

    // unfold the conditions vecs with its group number in groups
    // and find all possible combinations of the '#' groups
    // break early if the group combination is not valid - look ahead to make this decision

    // Find all possible combinations of the '#' groups
    // Each '#' in the condition is a group of 1, when not having neighbours of '#'
    // Each '?' can be replaced by either '#' or '.' and the groups can be combined
    // Any of the three symbol types can only be combined with neighbours of the same symbol type, and it will be a new group
    // The groups can only be combined in the order given in the groups list
    // The group numbers in the groups list are the sizes of the groups, and they give the size of groups of '#' only, in conditions list

    // start with looping through the conditions vecs
    let mut total: usize = 0;
    for (i, s_vec) in conditions.iter().enumerate() {
        // println!("{:?}", s_vec);
        // println!("{:?}", groups[count]);
        // println!("");
        total += count(&s_vec, &groups[i], &mut cache);
    }

    println!("Total valid combinations: {}", total);

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

pub fn part2(input: String) {
    // add memoization!!!!!
    // thanks HyperNeutrino.

    // start timer
    let now = Instant::now(); // mark time

    // part 2: now each condition vec of all vecs needs to be 5 times itself, separated by '?'
    // and the groups vecs need to be 5 times itself too, the rest of the calculation is the same

    let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
    let mut conditions: Vec<String> = Vec::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for line in input.lines() {
        // println!("{}", line);
        let mut parts = line.split(" ");
        // adapt to replicate conditions vec and groups vec 5 times each
        // therefore join the &str with '?' in between

        let single_str = parts.next().unwrap();
        conditions.push(
            std::iter::repeat(single_str)
                .take(5)
                .collect::<Vec<&str>>()
                .join("?"),
        );

        let mut temp_vec: Vec<usize> = Vec::new();
        let single_vec = parts
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        for _ in 0..5 {
            temp_vec.extend(single_vec.clone());
        }
        groups.push(temp_vec);
    }
    // println!("{:?}", conditions);
    // println!("{:?}", groups);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // start timer
    let now = Instant::now(); // mark time

    // unfold the conditions vecs with its group number in groups
    // and find all possible combinations of the '#' groups
    // break early if the group combination is not valid - look ahead to make this decision

    // Find all possible combinations of the '#' groups
    // Each '#' in the condition is a group of 1, when not having neighbours of '#'
    // Each '?' can be replaced by either '#' or '.' and the groups can be combined
    // Any of the three symbol types can only be combined with neighbours of the same symbol type, and it will be a new group
    // The groups can only be combined in the order given in the groups list
    // The group numbers in the groups list are the sizes of the groups, and they give the size of groups of '#' only, in conditions list

    // start with looping through the conditions vecs
    let mut total: usize = 0;
    for (i, s_vec) in conditions.iter().enumerate() {
        // println!("{:?}", s_vec);
        // println!("{:?}", groups[count]);
        // println!("");
        total += count(&s_vec, &groups[i], &mut cache);
    }

    println!("Total valid combinations: {}", total);

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
