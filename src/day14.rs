//! --- Day 14: Parabolic Reflector Dish ---
//! Roll all rocks North, O rocks are movable, # rocks are immovable
//! .'s are empty space
//! The total load is the sum of the load caused by all of the rounded rocks. In this example, the total load is 136.
//! Tilt the platform so that the rounded rocks all roll north. Afterward, what is the total load on the north support beams?
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

    let mut sum: i32 = 0;
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

    // now count all the O's in the pattern by its position reversed from bottom to top
    for (pos, row) in (1..=lines.len()).rev().zip(lines.iter()) {
        for col in row.iter() {
            if col == &'O' {
                sum += pos as i32;
            }
        }
    }

    println!("\nPart1 result is: {:?}", sum);
}
// 106371 too low

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
