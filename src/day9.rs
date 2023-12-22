use std::time::Instant;

pub fn diff(v1: Vec<i64>, v2: Vec<i64>) -> Vec<i64> {
    // calculates difference of 2 vectors - shifted by 1
    let res_vec = v1[0..(v1.len() - 1)]
        .to_vec()
        .into_iter()
        .zip(v2[1..].to_vec())
        .map(|(a, b)| b - a)
        .collect();
    return res_vec;
}

pub fn part1(input: String) {
    // let lines = txt.split("\n");
    let mut oasis: Vec<Vec<_>> = Vec::from(Vec::from([]));
    for line in input.lines() {
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

    // build "tree" -> next-curr=under -> stop if all are 0;
    // extrapolate: calculate B then A: reverse order -> append zero + ultimate line above = B, ...etc..., last of penultimate line + ultimate of 1st line = A
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

    // append 0 for all bottoms
    for i in 0..all_trees.len() {
        let end_pos: usize = all_trees[i].len() - 1;
        let end_pos_len: usize = all_trees[i][end_pos].len();
        all_trees[i][end_pos].splice(end_pos_len.., vec![0]);
    }

    for (i, _tree) in all_trees.clone().iter().enumerate() {
        // loop reverse through lines of each tree
        for line_pos in (0..(all_trees[i].len() - 1)).rev() {
            // extrapolation:
            // append zero + ultimate line above = B, ...etc..., last of penultimate line + ultimate of 1st line = A
            let last_pos_under = all_trees[i][line_pos + 1].len() - 1;
            let last_pos_above = all_trees[i][line_pos].len() - 1;
            let next_num: i64 = &all_trees[i][line_pos + 1][last_pos_under]
                + &all_trees[i][line_pos][last_pos_above];
            all_trees[i][line_pos].push(next_num);
        }
    }

    println!("after extrapolation: all trees {:?}", &all_trees);

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // sum each first's row last element
    let mut sum: i64 = 0;
    for i in 0..all_trees.len() {
        let end_pos: usize = all_trees[i][0].len() - 1;
        sum += all_trees[i][0][end_pos]
    }
    println!("\nThe result is: {:?}", sum);
}

pub fn part2(input: String) {
    // let lines = txt.split("\n");
    let mut oasis: Vec<Vec<_>> = Vec::from(Vec::from([]));
    for line in input.lines() {
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
