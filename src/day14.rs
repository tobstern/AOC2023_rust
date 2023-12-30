//! --- Day 14: Parabolic Reflector Dish ---
//! Roll all rocks North, O rocks are movable, # rocks are immovable
//! .'s are empty space
//! The total load is the sum of the load caused by all of the rounded rocks. In this example, the total load is 136.
//! Tilt the platform so that the rounded rocks all roll north. Afterward, what is the total load on the north support beams?

// use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashSet;
use std::time::Instant;

#[allow(unused)]
pub fn part1(input: String) {
    let mut lines: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    println!("lines = {:?}", &lines);

    // start timer
    let now = Instant::now(); // mark time

    // let mut sum: i32 = 0;
    // move every O rock up if possible
    // possible means, there is a . above it - else stop moving it (for # and if reached row == 0)
    let mut prev_pattern: Vec<Vec<char>> = lines.clone();
    // let mut curr_pattern: Vec<Vec<char>> = lines.clone();

    // loop until the pattern doesn't change
    'while_movable: loop {
        // move every O rock up if possible
        // possible means, there is a . above it - else stop moving it (for # and if reached row == 0)
        for row in 0..lines.len() {
            for col in 0..lines[row].len() {
                if lines[row][col] == 'O' {
                    // check if there is a . above it
                    if row > 0 && lines[row - 1][col] == '.' {
                        // move it up
                        lines[row - 1][col] = 'O';
                        lines[row][col] = '.';
                    }
                }
            }
        }

        // check if the pattern has changed
        if lines == prev_pattern {
            break 'while_movable;
        } else {
            prev_pattern = lines.clone();
        }
    }

    for line in lines.iter() {
        // println!("{:?}", line[0..40].to_vec());
        // println!();
        println!("{:?}", line.iter().collect::<String>());
    }

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let sum: i32 = score(&lines);

    println!("\nPart1 result is: {:?}", sum);
}

// function to rotate the pattern 90 degrees clockwise
#[allow(unused)]
pub fn rotate_90(lines: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rotated: Vec<Vec<char>> = vec![vec!['.'; lines.len()]; lines[0].len()];
    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            rotated[col][lines.len() - 1 - row] = lines[row][col];
        }
    }
    rotated
}

// function to move the rocks North
#[allow(unused)]
pub fn move_up(lines: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut moved: Vec<Vec<char>> = lines.clone();
    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            if lines[row][col] == 'O' {
                // check if there is a . above it
                if row > 0 && lines[row - 1][col] == '.' {
                    // move it up
                    moved[row - 1][col] = 'O';
                    moved[row][col] = '.';
                }
            }
        }
    }
    moved
}

fn score(lines: &Vec<Vec<char>>) -> i32 {
    let mut sum: i32 = 0;
    // now count all the O's in the pattern by its position reversed from bottom to top
    for (pos, row) in (1..=lines.len()).rev().zip(lines.iter()) {
        for col in row.iter() {
            if col == &'O' {
                sum += pos as i32;
            }
        }
    }
    sum
}

#[allow(unused)]
pub fn part2(input: String) {
    let mut lines: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    // println!("lines = {:?}", &lines);

    // start timer
    let now = Instant::now(); // mark time

    // part 2:
    // Each cycle tilts the platform four times so that the rounded rocks roll north, then west, then south, then east. After each tilt, the rounded rocks roll as far as they can before the platform tilts in the next direction. After one cycle, the platform will have finished rolling the rounded rocks in those four directions in that order.
    // tilt: N, W, S, E
    let mut rounds: usize = 1000000000;

    // let tilt: Vec<char> = vec!['N', 'W', 'S', 'E'];
    let mut period: usize = 1;
    let mut patterns: HashSet<Vec<Vec<char>>> = HashSet::new();
    let mut patterns_vec: Vec<(usize, Vec<Vec<char>>)> = Vec::new();
    let mut prev_pattern: Vec<Vec<char>> = lines.clone();

    let row_len: usize = lines.len();
    let col_len: usize = lines[0].len();

    let mut round: usize = 0;
    'rounds: loop {
        round += 1;

        // do the cycle (roll + tilt 90° 4 times)
        for i in 0..4 {
            // break if hit period activated and reached rounds limit
            if round >= rounds {
                break 'rounds;
            }

            // loop until the pattern doesn't change (in the current direction)
            'while_movable: loop {
                // move every O rock up if possible
                // possible means, there is a . above it - else stop moving it (for # and if reached row == 0)

                // move every O rock up if possible
                lines = move_up(&lines);

                if lines == prev_pattern {
                    break 'while_movable;
                } else {
                    prev_pattern = lines.clone();
                }
            }
            // rotate the pattern 90 degrees clockwise
            lines = rotate_90(&lines);
        }

        // if pattern repeats, then we can skip the rest of the rounds
        // if pattern doesn't repeat, then we can continue with the next cycle
        // check if the pattern has changed

        // check first if duplicates are in pattern_collection
        // if so, the positions of them are the start and end of the period
        if !patterns.insert(lines.clone()) {
            // get the index of the first duplicate
            let first_dup = patterns_vec
                .iter()
                .position(|x: (&(usize, Vec<Vec<char>>))| x.1 == lines)
                .expect("first duplicate not found");
            // found a duplicate
            let first: usize = patterns_vec[first_dup].0;

            // calculate the period
            period = round - first;

            // go in period steps until 1e9, but need to find the offset value of rounds
            let offset = (rounds - first) % (period) + first;

            lines = patterns_vec[offset - 1].1.clone();

            break 'rounds;
        }

        // save for pattern check
        patterns.insert(lines.clone());
        patterns_vec.push((round, lines.clone()));
    }

    let sum: i32 = score(&lines);

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("\nPart2 result is: {:?}", sum);
}
