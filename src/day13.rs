//! --- Day 13: Point of Incidence ---
//! To summarize your pattern notes, add up the number of columns to the left of each vertical line of reflection; to that, also add 100 multiplied by the number of rows above each horizontal line of reflection. In the above example, the first pattern's vertical line has 5 columns to its left and the second pattern's horizontal line has 4 rows above it, a total of 405.

//! Find the line of reflection in each of the patterns in your notes. What number do you get after summarizing all of your notes?
use std::time::Instant;

#[allow(unused)]
pub fn part1(input: String) {
    let lines = input.split("\n");

    let line_vec: Vec<&str> = lines.collect();

    // start timer
    let now = Instant::now(); // mark time

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // println!("\nPart1 result is: {:?}", sum);
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
