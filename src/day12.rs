//! --- Day 12: Hot Springs ---
//!
//! You finally reach the hot springs! You can see steam rising from secluded areas attached to the primary, ornate building.
//!
//! As you turn to enter, the researcher stops you. "Wait - I thought you were looking for the hot springs, weren't you?" You indicate that this definitely looks like hot springs to you.
//!
//! "Oh, sorry, common mistake! This is actually the onsen! The hot springs are next door."
//!
//! You look in the direction the researcher is pointing and suddenly notice the massive metal helixes towering overhead. "This way!"
//!
//! It only takes you a few more steps to reach the main gate of the massive fenced-off area containing the springs. You go through the gate and into a small administrative building.
//!
//! "Hello! What brings you to the hot springs today? Sorry they're not very hot right now; we're having a lava shortage at the moment." You ask about the missing machine parts for Desert Island.
//!
//! "Oh, all of Gear Island is currently offline! Nothing is being manufactured at the moment, not until we get more lava to heat our forges. And our springs. The springs aren't very springy unless they're hot!"
//!
//! "Say, could you go up and see why the lava stopped flowing? The springs are too cold for normal operation, but we should be able to find one springy enough to launch you up there!"
//!
//! There's just one problem - many of the springs have fallen into disrepair, so they're not actually sure which springs would even be safe to use! Worse yet, their condition records of which springs are damaged (your puzzle input) are also damaged! You'll need to help them repair the damaged records.
//!
//! In the giant field just outside, the springs are arranged into rows. For each row, the condition records show every spring and whether it is operational (.) or damaged (#). This is the part of the condition records that is itself damaged; for some springs, it is simply unknown (?) whether the spring is operational or damaged.
//!
//! However, the engineer that produced the condition records also duplicated some of this information in a different format! After the list of springs for a given row, the size of each contiguous group of damaged springs is listed in the order those groups appear in the row. This list always accounts for every damaged spring, and each number is the entire size of its contiguous group (that is, groups are always separated by at least one operational spring: #### would always be 4, never 2,2).
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

use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::cmp::max;
use std::time::Instant;

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
pub fn generate_combinations_rec<F: FnMut(&Vec<char>)>(
    s: &mut Vec<char>,
    i: usize,
    mut callback: F,
) {
    if i == s.len() {
        callback(s);
        return;
    }

    if s[i] == '?' {
        s[i] = '#';
        generate_combinations_rec(s, i + 1, &mut callback);
        s[i] = '.';
        generate_combinations_rec(s, i + 1, &mut callback);
        s[i] = '?';
    } else {
        generate_combinations_rec(s, i + 1, &mut callback);
    }
}

#[allow(unused)]
pub fn generate_combinations_iter<F: FnMut(&Vec<char>) -> bool>(
    s: &mut Vec<char>,
    groups: &Vec<usize>,
    mut callback: F,
) -> usize {
    let mut valid_combinations: usize = 0;
    let mut stack: Vec<(usize, Vec<char>, usize, usize)> = vec![(0, s.clone(), 0, 0)];

    while let Some((i, current, group_index, group_size)) = stack.pop() {
        if i == current.len() {
            if group_index == groups.len() && callback(&current) {
                valid_combinations += 1;
            }
        } else if current[i] == '?' {
            let mut next: Vec<char> = current.clone();
            next[i] = '#';
            let mut next_group_index = group_index;
            let mut next_group_size = group_size + 1;
            if i == 0 || current[i - 1] != '#' {
                if group_index < groups.len() && next_group_size == groups[group_index] {
                    next_group_index += 1;
                    next_group_size = 0;
                } else {
                    continue;
                }
            }
            stack.push((i + 1, next.clone(), next_group_index, next_group_size));

            next[i] = '.';
            stack.push((i + 1, next.clone(), group_index, group_size));
        } else {
            stack.push((i + 1, current, group_index, group_size));
        }
    }
    valid_combinations
}

#[allow(unused)]
pub fn count_valid_combinations(s: &Vec<char>, groups: &Vec<usize>) -> usize {
    let n = s.len();
    let m = groups.len();
    let mut dp = vec![vec![vec![0; n + 1]; m + 1]; n + 1];
    dp[0][0][0] = 1;
    for i in 0..n {
        for j in 0..=m {
            for k in 0..=n {
                if s[i] == '#' {
                    if k > 0 {
                        dp[i + 1][j][k] = dp[i][j][k - 1]; // extend the current group
                    }
                } else {
                    dp[i + 1][j][0] = dp[i][j][k]; // end the current group
                    if j < m && k == groups[j] {
                        dp[i + 1][j + 1][0] = dp[i][j][k]; // start a new group
                    }
                }
            }
        }
    }
    dp[n][m][0]
}
// pub fn count_valid_combinations(s: &Vec<char>, groups: &Vec<usize>) -> usize {
//     let n = s.len();
//     let m = groups.len();
//     let mut dp = vec![vec![vec![0; n + 1]; m + 1]; n + 1];
//     dp[0][0][0] = 1;
//     for i in 0..n {
//         for j in 0..=m {
//             for k in 0..=n {
//                 if s[i] != '#' {
//                     dp[i + 1][j][k] += dp[i][j][k]; // extend the current group
//                 }
//                 if s[i] != '.' && j < m && k == groups[j] {
//                     dp[i + 1][j + 1][0] += dp[i][j][k]; // start a new group
//                 }
//             }
//         }
//     }
//     dp[n][m][0]
// }

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

#[allow(unused)]
pub fn part2(input: String) {
    //
    // start timer
    let now = Instant::now(); // mark time

    // part 2: now each condition vec of all vecs needs to be 5 times itself, separated by '?'
    // and the groups vecs need to be 5 times itself too, the rest of the calculation is the same

    let mut conditions: Vec<Vec<char>> = Vec::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for line in input.lines() {
        // println!("{}", line);
        let mut parts = line.split(" ");
        // adapt to replicate conditions vec and groups vec 5 times each
        let mut temp_vec: Vec<char> = Vec::new();
        let mut single_vec = parts.next().unwrap().chars().collect::<Vec<char>>();
        for _ in 0..5 {
            temp_vec.extend(single_vec.clone());
            temp_vec.push('?');
        }
        // remove last '?'
        temp_vec.pop();
        conditions.push(temp_vec);

        let mut temp_vec: Vec<usize> = Vec::new();
        let mut single_vec = parts
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
    println!("{:?}", conditions);
    println!("{:?}", groups);

    // start timer
    let now = Instant::now(); // mark time

    // Find all possible combinations of the '#' groups
    // Each '#' in the condition is a group of 1, when not having neighbours of '#'
    // Each '?' can be replaced by either '#' or '.' and the groups can be combined
    // Any of the three symbol types can only be combined with neighbours of the same symbol type, and it will be a new group
    // The groups can only be combined in the order given in the groups list
    // The group numbers in the groups list are the sizes of the groups, and they give the size of groups of '#' only, in conditions list

    let mut count: usize = 0;
    let mut total_valid_combinations: usize = 0;

    let total_conditions = conditions.len();
    let pb = ProgressBar::new(total_conditions as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .progress_chars("#>-"),
    );

    let total_valid_combinations: usize = conditions
        .par_iter()
        .enumerate()
        .map(|(count, s_vec)| {
            let result = count_valid_combinations(s_vec, &groups[count]);
            pb.inc(1);
            result
        })
        .sum();

    // let total_valid_combinations: usize = conditions
    //     .par_iter_mut()
    //     .enumerate()
    //     .map(|(count, s_vec)| {
    //         let result = generate_combinations_iter(s_vec, &groups[count], |combination| {
    //             let mut groups_vec: Vec<usize> = Vec::new();
    //             let mut group_size: usize = 0;
    //             for c in combination {
    //                 if c == &'#' {
    //                     group_size += 1;
    //                 } else if group_size > 0 {
    //                     groups_vec.push(group_size);
    //                     group_size = 0;
    //                 }
    //             }
    //             if group_size > 0 {
    //                 groups_vec.push(group_size);
    //             }
    //             groups_vec == groups[count]
    //         });
    //         pb.inc(1);
    //         result
    //     })
    //     .sum();

    pb.finish_with_message("done");
    // let total_valid_combinations: usize = conditions
    //     .par_iter_mut()
    //     .enumerate()
    //     .map(|(count, s_vec)| {
    //         generate_combinations_iter(s_vec, |combination| {
    //             let mut groups_vec: Vec<usize> = Vec::new();
    //             let mut group_size: usize = 0;
    //             for c in combination {
    //                 if c == &'#' {
    //                     group_size += 1;
    //                 } else if group_size > 0 {
    //                     groups_vec.push(group_size);
    //                     group_size = 0;
    //                 }
    //             }
    //             if group_size > 0 {
    //                 groups_vec.push(group_size);
    //             }

    //             // println!("groups_vec {:?}", &groups_vec);
    //             // println!("groups @count {:?}", &groups[count]);
    //             groups_vec == groups[count]
    //         })
    //     })
    //     .sum();
    // for mut s_vec in conditions {
    //     total_valid_combinations += generate_combinations_iter(&mut s_vec, |combination| {
    //         let mut groups_vec: Vec<usize> = Vec::new();
    //         let mut group_size: usize = 0;
    //         for c in combination {
    //             if c == &'#' {
    //                 group_size += 1;
    //             } else if group_size > 0 {
    //                 groups_vec.push(group_size);
    //                 group_size = 0;
    //             }
    //         }
    //         if group_size > 0 {
    //             groups_vec.push(group_size);
    //         }

    //         println!("groups_vec {:?}", &groups_vec);
    //         println!("groups @count {:?}", &groups[count]);
    //         groups_vec == groups[count]
    //     });
    //     count += 1;
    // }

    println!("Total valid combinations: {}", total_valid_combinations);

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
