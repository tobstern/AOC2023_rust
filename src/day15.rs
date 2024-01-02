//! --- Day 15: Lens Library ---
//! The HASH algorithm is a way to turn any string of characters into a single number in the range 0 to 255. To run the HASH algorithm on a string, start with a current value of 0. Then, for each character in the string starting from the beginning:
//! Determine the ASCII code for the current character of the string.
//! Increase the current value by the ASCII code you just determined.
//! Set the current value to itself multiplied by 17.
//! Set the current value to the remainder of dividing itself by 256.
//!
//! After following these steps for each character in the string in order, the current value is the output of the HASH algorithm.
//!
use std::collections::HashMap;
use std::time::Instant;

pub fn ascii(ch: char) -> usize {
    ch as usize
}

#[allow(unused)]
pub fn part1(input: String) {
    let mut result: usize = 0;
    // println!("test ascii_sum = {}", ascii_sum("HASH"));
    for string in input.trim().split(",") {
        let mut curr_val: usize = 0;
        for ch in string.chars() {
            curr_val += ascii(ch);
            curr_val *= 17;
            curr_val %= 256;
        }
        result += curr_val;
    }

    // start timer
    let now = Instant::now(); // mark time

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("\nPart1 result is: {:?}", result);
}

// part 1 as function for part 2
fn get_hash(string: &str) -> usize {
    let mut curr_val: usize = 0;
    for ch in string.chars() {
        curr_val += ascii(ch);
        curr_val *= 17;
        curr_val %= 256;
    }
    curr_val
}

fn focus_power(boxes: &HashMap<usize, Vec<(&str, usize)>>) -> usize {
    // println!(
    //     "all boxes: {:?}",
    //     &boxes
    //         .iter()
    //         .filter(|(_pos, vec)| vec.len() > 0)
    //         .collect::<Vec<_>>()
    // );
    let mut sum: usize = 0;
    for (box_num, vec) in boxes {
        for (slot_num, (_, lense_num)) in vec.iter().enumerate() {
            sum += (box_num + 1) * (slot_num + 1) * lense_num;
        }
    }
    sum
}

#[allow(unused)]
pub fn part2(input: String) {
    // If the operation character is a dash (-), go to the relevant box and remove the lens with the given label if it is present in the box. Then, move any remaining lenses as far forward in the box as they can go without changing their order, filling any space made by removing the indicated lens. (If no lens in that box has the given label, nothing happens.)
    // If the operation character is an equals sign (=), it will be followed by a number indicating the focal length of the lens that needs to go into the relevant box; be sure to use the label maker to mark the lens with the label given in the beginning of the step so you can find it later. There are two possible situations:
    //
    // If there is already a lens in the box with the same label, replace the old lens with the new lens: remove the old lens and put the new lens in its place, not moving any other lenses in the box.
    // If there is not already a lens in the box with the same label, add the lens to the box immediately behind any lenses already in the box. Don't move any of the other lenses when you do this. If there aren't any lenses in the box, the new lens goes all the way to the front of the box.
    let mut boxes: HashMap<usize, Vec<(&str, usize)>> = HashMap::new();
    let inner_vec: Vec<(&str, usize)> = Vec::new();

    // initialize all boxes
    for pos in 0..256 {
        boxes.insert(pos, inner_vec.clone());
    }

    'all_lenses: for string in input.trim().split(",") {
        if string.contains("=") {
            // insert lens
            let s = string.split("=").next().unwrap(); // get label
                                                       // get lense number
            let lense_num: usize = string.split("=").last().unwrap().parse().unwrap();
            // println!("inserting lens: {} with lense_num: {}", &s, &lense_num);

            let k = get_hash(s);

            let curr_vec: Option<&mut Vec<(&str, usize)>> = boxes.get_mut(&k);

            let exists = match &curr_vec {
                Some(x) => true,
                _ => {
                    println!("WTF, should be something, right?");
                    false
                }
            };

            if exists {
                // if reached here, lense exists -> remove it
                let mut vec = curr_vec.unwrap();
                for (i, (label, _focal_num)) in vec.iter().enumerate() {
                    if *label == s {
                        // vec.remove(i);
                        vec[i] = (s, lense_num);

                        // overwrite HashMap @k
                        continue 'all_lenses;
                    }
                }

                // if not already in there: add it
                // vec is mutable, so we can push to it
                vec.push((s, lense_num));
            }
            let mut curr_vec = boxes
                .get(&k)
                .expect("This key is not in boxes - box not found?");
        } else if string.contains("-") {
            // remove lens
            // get label
            let s = string.split("-").next().unwrap();
            // println!("removing lens: {}", &s);
            let k = get_hash(s);

            let curr_vec: Option<&mut Vec<(&str, usize)>> = boxes.get_mut(&k);

            let exists = match &curr_vec {
                Some(x) => true,
                _ => {
                    println!("WTF, should be something, right?");
                    false
                }
            };

            if exists {
                // if reached here, lense exists -> remove it
                let mut vec = curr_vec.unwrap();
                for (i, (label, _)) in vec.iter().enumerate() {
                    if *label == s {
                        vec.remove(i);

                        // overwrite HashMap @k
                        continue 'all_lenses;
                    }
                }
            }
        }
    }

    // start timer
    let now = Instant::now(); // mark time

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("\nPart2 result is: {:?}", focus_power(&boxes));
}

//
// 514713 too high
