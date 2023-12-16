use std::fs;
use std::time::Instant;

pub fn read_txt(day: String) -> String {
    // read in puzzle input: dayXX.txt
    let suffix: &str = ".txt";

    let file_path: String = "puzzle_inputs/day".to_owned() + &day + &suffix;

    let text: String = fs::read_to_string(file_path).expect("Could not open the text-file");

    return text;
}

fn diff(v1: Vec<i64>, v2: Vec<i64>) -> Vec<i64> {
    // calculates difference of 2 vectors - shifted by 1
    let res_vec = v1[0..(v1.len() - 1)]
        .to_vec()
        .into_iter()
        .zip(v2[1..].to_vec())
        .map(|(a, b)| b - a)
        .collect();
    return res_vec;
}

fn main() {
    let day = String::from("09");
    // let day = String::from("09_test");

    // read in the text-file
    let txt: String = read_txt(day);

    // let lines = txt.split("\n");
    let mut oasis: Vec<Vec<_>> = Vec::from(Vec::from([]));
    for line in txt.lines() {
        // split at whitespaces
        oasis.push(
            line.split_whitespace()
                .map(|x| x.parse::<i64>().ok().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    println!("oasis sensor data {:?}", &oasis);
    // parsed the input

    // start timer
    let now = Instant::now(); // mark time

    // part 2:
    // build "tree" -> next-curr=upper -> stop if all are 0;
    // extrapolate: (line above - prepended zero = B), ...etc..., 1st of 1st line - 1st of penultimate line = A
    // all A's sum: result.
    let mut all_trees: Vec<Vec<Vec<_>>> = Vec::from(Vec::from(Vec::from([])));
    for (_i, sense_vec) in oasis.iter().cloned().enumerate() {
        // for every sensing vec, do extrapolation

        // initial line is already there -> calc under:
        let mut tree: Vec<Vec<_>> = Vec::from(Vec::from([]));
        tree.push(sense_vec);

        let mut line_count: usize = 0;
        loop {
            // println!("current tree {:?}", &tree);

            // loop until all are 0
            if tree[line_count].iter().all(|x| x == &0) {
                // reached last line - end!
                break;
            }

            // calc next - curr = under
            // println!("vec for diff {:?}", &tree[line_count].clone());
            // println!(
            //     "diff result {:?}",
            //     &diff(tree[line_count].clone(), tree[line_count].clone())
            // );
            tree.push(diff(tree[line_count].clone(), tree[line_count].clone()));

            line_count += 1;
        }
        // save the built up tree
        all_trees.push(tree.clone());
    }

    println!("all trees {:?}", &all_trees);

    // ! prepend 0 for all bottoms
    for i in 0..all_trees.len() {
        let end_pos: usize = all_trees[i].len() - 1;
        all_trees[i][end_pos].splice(0..0, vec![0]);
    }

    for (i, _tree) in all_trees.clone().iter().enumerate() {
        // loop reverse through lines of each tree
        for line_pos in (0..(all_trees[i].len() - 1)).rev() {
            // extrapolation for part 2:
            let next_num: i64 = &all_trees[i][line_pos][0] - &all_trees[i][line_pos + 1][0];
            all_trees[i][line_pos].splice(0..0, vec![next_num]);
        }
    }

    println!("after extrapolation: all trees {:?}", &all_trees);

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // sum each first's row last element
    let mut sum: i64 = 0;
    for i in 0..all_trees.len() {
        sum += all_trees[i][0][0];
    }
    println!("\nThe result is: {:?}", sum);
}