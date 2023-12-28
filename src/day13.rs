//! --- Day 13: Point of Incidence ---
//! To summarize your pattern notes, add up the number of columns to the left of each vertical line of reflection; to that, also add 100 multiplied by the number of rows above each horizontal line of reflection. In the above example, the first pattern's vertical line has 5 columns to its left and the second pattern's horizontal line has 4 rows above it, a total of 405.
//! Find the line of reflection in each of the patterns in your notes. What number do you get after summarizing all of your notes?
use super::day11::transpose;
use std::time::Instant;

// Function to check for mirror positions in a vector of strings
fn check_mirror_positions(lines: &[String]) -> Vec<usize> {
    let mut mirror_positions = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if i + 1 < lines.len() && line == &lines[i + 1] {
            mirror_positions.push(i + 1);
        }
    }

    mirror_positions
}

// Function to check if a mirror position is valid in a vector of strings
fn valid_mirror_positions(lines: &[String], mirror_position: usize) -> (bool, Vec<usize>) {
    let mut track_lines: Vec<usize> = Vec::new();
    for i in 0..lines.len() {
        println!("current posistion: {:?}", &i);

        if mirror_position + i >= lines.len() || (mirror_position as i32) - (i as i32) - 1 < 0 {
            println!("Does 'overflow in index' ever happen?");
            println!(
                "upper {}, lower {}",
                mirror_position + i,
                (mirror_position as i32) - (i as i32) - 1
            );
            println!("track_lines {:?}", &track_lines);
            return (true, track_lines);
        }

        if &lines[mirror_position - i - 1] != &lines[mirror_position + i] {
            println!("Does 'no matched lines' ever happen?");
            println!(
                "line {:?} is not same as next line {:?}",
                &lines[mirror_position - i - 1],
                &lines[mirror_position + i]
            );

            println!("stopped @invalid, mirror position {:?}", &mirror_position);

            return (false, track_lines);
        } else {
            track_lines.push(mirror_position - i - 1);
            track_lines.push(mirror_position + i);
        }
    }
    // false
    (false, track_lines)
}

#[allow(unused)]
pub fn part1(input: String) {
    let blocks_lines: Vec<Vec<String>> = input
        .split("\n\n")
        .map(|x| {
            x.trim()
                .lines()
                .map(|x| x.trim().to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("blocks lines {:?}", &blocks_lines);

    let mut blocks_cols: Vec<Vec<Vec<char>>> = Vec::new();

    for block in &blocks_lines {
        blocks_cols.push(transpose(
            block
                .clone()
                .iter()
                .map(|x| x.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        ));
    }

    // println!("blocks cols with chars {:?}", &blocks_cols);

    let blocks_cols_str: Vec<Vec<String>> = blocks_cols
        .iter()
        .map(|x| {
            x.iter()
                .map(|x| x.iter().cloned().collect::<String>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("blocks cols {:?}", &blocks_cols_str);

    // start timer
    let now = Instant::now(); // mark time

    // check for every line if same as next line
    // also transpose the lines to check for columns (clone of lines)
    // if same save current index + 1 as mirror position (append to vec)
    // if only 1 pos, found mirror - else, there can be multiple mirrors in one block - so need to check which is biggest
    // check from mirror position, compare mirror_pos - 1 with mirror_pos + 1, till, not same, or end of line - then stop!
    // final result is: sum of mirror positions (columns) + 100 * sum of mirror positions (lines)

    let mut sum: usize = 0;
    let mut c: usize = 0;
    'blocks: loop {
        // debug purpose:
        if c >= blocks_lines.len() {
            break 'blocks;
        }

        println!("\n\nNew block: {:?}", &c);

        let mut valid_mirrors: Vec<usize> = Vec::new();

        // first loop over lines
        let mut mirror_pos_line = check_mirror_positions(&blocks_lines[c]);

        let mut mirror_pos_col = check_mirror_positions(&blocks_cols_str[c]);

        // found mirror positions for this block
        // it should be either in mirror_pos_line or mirror_pos_col!
        println!("mirror_pos_line {:?}", &mirror_pos_line);
        println!("mirror_pos_col {:?}", &mirror_pos_col);

        // check if all but one lines are same, break immediatly if not
        // valid mirror lines/cols for this block
        let mut pot_mirror_line: (usize, usize) = (0, 0);

        let mut temp: Vec<(usize, usize)> = Vec::new();
        for mirr_pos in &mirror_pos_line {
            // first lines
            let mirror_line: (bool, Vec<_>) = valid_mirror_positions(&blocks_lines[c], *mirr_pos);

            if mirror_line.0 {
                // valid mirror position
                temp.push((*mirr_pos, mirror_line.1.len()));
            }
        }
        println!("temp {:?}", &temp);

        // if &mirror_pos_line.len() < &1 {
        if &temp.len() < &1 {
            // posis are empty, so no mirror in lines
            temp.push((0, 0));
        } else {
            temp.sort_by_key(|k: &(usize, usize)| k.1);
            pot_mirror_line = *temp.iter().rev().collect::<Vec<_>>()[0];
        }

        let mut pot_mirror_col: (usize, usize) = (0, 0);

        let mut temp: Vec<(usize, usize)> = Vec::new();
        for mirr_pos in &mirror_pos_col {
            // then columns
            let mirror_col: (bool, Vec<_>) = valid_mirror_positions(&blocks_cols_str[c], *mirr_pos);

            if mirror_col.0 {
                // valid mirror position
                temp.push((*mirr_pos, mirror_col.1.len()));
            }
        }
        println!("temp {:?}", &temp);

        // if &mirror_pos_col.len() < &1 {
        if &temp.len() < &1 {
            // posis are empty, so no mirror in lines
            temp.push((0, 0));
        } else {
            temp.sort_by_key(|k: &(usize, usize)| k.1);
            pot_mirror_col = *temp.iter().rev().collect::<Vec<_>>()[0];
        }

        println!("pot_mirror_line {:?}", &pot_mirror_line);
        println!("pot_mirror_col {:?}", &pot_mirror_col);
        // now valid it is, for the biggest mirror - there should be one either in lines or cols!
        if pot_mirror_line.1 > pot_mirror_col.1 {
            sum += pot_mirror_line.0 * 100;
        } else {
            sum += pot_mirror_col.0;
        };

        // go to next block
        c += 1;
    }

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("\nPart1 result is: {:?}", sum);
}

pub fn check_for_smudge(lower: &String, upper: &String) -> bool {
    // consider now the smudge:
    // if only one char is different, then it is valid
    let smudges: Vec<usize> = lower
        .chars()
        .zip(upper.chars())
        .enumerate()
        .filter_map(|(i, (a, b))| {
            if a != b {
                // println!("smudge at position {:?}", &i);
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // now check if only one char is different
    if smudges.len() == 1 {
        println!(
            "There is a smudge! for \nlower {:?} and \nupper {:?}",
            &lower, &upper
        );
        return true;
    } else {
        return false;
    }
}

// part 2:
// Function to check for mirror positions in a vector of strings
fn check_mirror_positions2(lines: &[String]) -> Vec<usize> {
    let mut mirror_positions = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if mirror_positions.len() > 0 {
            // if there is already a mirror position, then break
            // only find 1st mirror pos?
            //break;
        }
        if i + 1 >= lines.len() {
            break;
        }

        if line == &lines[i + 1] {
            mirror_positions.push(i + 1);
        } else {
            // consider now the smudge:
            // if only one char is different, then it is valid
            let is_smudge = check_for_smudge(&lines[i], &lines[i + 1]);

            if is_smudge {
                // is valid
                mirror_positions.push(i + 1);
            }
        }
    }

    mirror_positions
}

// for part2:
// Function to check if a mirror position is valid in a vector of strings
// mirrors can have a smudge, i.e. one line is not same as next line, but only 1 char is different
fn valid_mirror_positions2(lines: &[String], mirror_position: usize) -> bool {
    let mut smudge_count: usize = 0;
    for i in 0..lines.len() {
        println!("current posistion: {:?}", &i);

        if mirror_position + i >= lines.len() || (mirror_position as i32) - (i as i32) - 1 < 0 {
            // more than one smudge, so, it is invalid
            // there is exactly one smudge per mirror!
            println!("hit the end of the line, so invalid");
            if smudge_count == 1 {
                return true;
            } else {
                return false;
            }
            // return false;
        }

        let upper_str = &lines[mirror_position + i];
        let lower_str = &lines[mirror_position - i - 1];

        let is_smudge: bool = check_for_smudge(&lower_str, &upper_str);

        if is_smudge {
            // return true;
            smudge_count += 1;
        }
    }
    false
}

#[allow(unused)]
pub fn part2(input: String) {
    let blocks_lines: Vec<Vec<String>> = input
        .split("\n\n")
        .map(|x| {
            x.trim()
                .lines()
                .map(|x| x.trim().to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("blocks lines {:?}", &blocks_lines);

    let mut blocks_cols: Vec<Vec<Vec<char>>> = Vec::new();

    for block in &blocks_lines {
        blocks_cols.push(transpose(
            block
                .clone()
                .iter()
                .map(|x| x.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        ));
    }

    // println!("blocks cols with chars {:?}", &blocks_cols);

    let blocks_cols_str: Vec<Vec<String>> = blocks_cols
        .iter()
        .map(|x| {
            x.iter()
                .map(|x| x.iter().cloned().collect::<String>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("blocks cols {:?}", &blocks_cols_str);

    // start timer
    let now = Instant::now(); // mark time

    // check for every line if same as next line
    // also transpose the lines to check for columns (clone of lines)
    // if same save current index + 1 as mirror position (append to vec)
    // if only 1 pos, found mirror - else, there can be multiple mirrors in one block - so need to check which is biggest
    // check from mirror position, compare mirror_pos - 1 with mirror_pos + 1, till, not same, or end of line - then stop!
    // final result is: sum of mirror positions (columns) + 100 * sum of mirror positions (lines)

    // part2:
    // find the smudge, i.e. one line is not same as next line, but only 1 char is different
    // 1st consider lines, then columns, there appears to be that kind of hierarchy.

    let mut sum: usize = 0;
    let mut c: usize = 0;
    'blocks: loop {
        // debug purpose:
        if c >= blocks_lines.len() {
            break 'blocks;
        }

        println!("\n\nNew block: {:?}", &c + &1);

        let mut valid_mirrors: Vec<usize> = Vec::new();

        // first loop over lines
        let mut mirror_pos_line = check_mirror_positions2(&blocks_lines[c]);

        let mut mirror_pos_col = check_mirror_positions2(&blocks_cols_str[c]);

        // found mirror positions for this block
        // it should be either in mirror_pos_line or mirror_pos_col!
        println!("mirror_pos_line {:?}", &mirror_pos_line);
        println!("mirror_pos_col {:?}", &mirror_pos_col);

        // check if all but one lines are same, break immediatly if not
        // valid mirror lines/cols for this block
        let mut mirror_line: bool = false;
        if mirror_pos_line.len() > 0 {
            let mirr_pos: usize = mirror_pos_line[0];

            // first lines
            mirror_line = valid_mirror_positions2(&blocks_lines[c], mirr_pos);

            if mirror_line {
                // valid mirror position
                sum += mirr_pos * 100;
                // go to next block
                c += 1;
                continue 'blocks;
            }
            println!("mirror position {:?}, is valid={}", &mirr_pos, &mirror_line);
        } else {
            // no mirror in lines
            println!("no mirror in lines");
        }

        if mirror_pos_col.len() > 0 {
            let mirr_pos: usize = mirror_pos_col[0];

            // only consider columns if temp of lines is empty
            // then columns
            let mirror_col: bool = valid_mirror_positions2(&blocks_cols_str[c], mirr_pos);

            if mirror_col {
                sum += mirr_pos;
                // go to next block
                c += 1;
                continue 'blocks;
            }
            println!("mirror position {:?}, is valid={}", &mirr_pos, &mirror_col);
        } else {
            // no mirror in lines
            println!("no mirror in columns");
        }

        // go to next block
        c += 1;
    }

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("\nPart2 result is: {:?}", sum);
}

// 45619 too high
// 33916 too high
//
// 21740 not correct
// 21725 too low
