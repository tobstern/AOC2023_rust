// use std::collections::HashSet;
use std::fs;
use std::time::Instant;

// #[derive(Debug)]
pub fn read_txt(day: String) -> String {
    // read in puzzle input: dayXX.txt
    let suffix: &str = ".txt";

    let file_path: String = "puzzle_inputs/day".to_owned() + &day + &suffix;

    let text: String = fs::read_to_string(file_path).expect("Could not open the text-file");

    return text;
}

fn shift_range(range: &Vec<i64>, offset: &i64) -> Vec<i64> {
    // shifts the given range by offset, and gives it back

    // map the start:
    let mut next_range: Vec<i64> = Vec::from([]);

    // shift start of destinat by delta -> new start
    next_range.push(&range[2] + offset);

    // shift destination end by delta -> new end
    next_range.push(&range[3] + offset);

    // println!("return next_range {:?}", &next_range);

    return next_range;
}

fn main() {
    // let day = String::from("05");
    let day = String::from("05_test");

    // read in the text-file
    let txt: String = read_txt(day);

    let blocks = txt.split("\n\n");

    let mut map: Vec<Vec<_>> = Vec::new();

    // cut blocks @\n
    for lines in blocks.collect::<Vec<_>>().iter() {
        // split each and save them into block_line_vec
        map.push(
            lines
                .clone()
                .split("\n")
                .map(|x: &str| {
                    x.split(" ")
                        .filter_map(|s| s.parse::<i64>().ok())
                        .collect::<Vec<i64>>()
                })
                // .inspect(|x: &Vec<_>| println!("lines have been split --> {:?}", x))
                .collect(),
        );
    }
    // println!("{:?}", &map);

    // input has been parsed:

    let now = Instant::now(); // mark time

    let seeds_raw: Vec<_> = map[0][0].iter().copied().collect();

    // switch to ranges: convert to: [dest, src, range] -> [src_start, src_end, dest_start, dest_end]
    let mut seeds: Vec<Vec<i64>> = Vec::from(Vec::from([]));
    for (i, ele) in seeds_raw.iter().enumerate() {
        // make ranges, so convert: [seed_start, range] -> [seed_start, seed_end]
        // remark: ranges are including end!

        if i % 2 == 0 {
            seeds.push(vec![*ele, (ele + seeds_raw[i + 1])]);
        }
    }

    let mut map: Vec<_> = map[1..(map.len())]
        .into_iter()
        .map(|x| x.into_iter().cloned().collect::<Vec<_>>())
        .collect();

    // change to ranges:
    let map_iter: Vec<Vec<_>> = map.iter().cloned().collect();
    for (i, block) in map_iter.iter().enumerate() {
        for (j, line) in block.iter().enumerate() {
            if &map_iter[i][j].len() < &1 {
                continue;
            }
            // make ranges, so convert: [dest, src, range] -> [src_start, src_end, dest_start, dest_end]
            // remark: ranges are including end!
            map[i][j] = vec![
                line[1],
                line[1] + line[2] - 1,
                line[0],
                line[0] + line[2] - 1,
            ];
        }
    }

    // seeds = all_seeds.iter().flatten().map(|x| *x).collect::<Vec<i64>>();
    println!("seeds: {:?}", &seeds);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("seeds {:?}", seeds);
    println!("blocks {:?}", &map);

    let now = Instant::now();
    // look up each of the seeds (values) -> find its location number -> min() is result!
    // let mut results: Vec<Vec<i64>> = vec![vec![]];
    let mut results: Vec<Vec<i64>> = Vec::from(Vec::from([]));

    for seed in seeds {
        // saved as [destination, source, range]
        // loop through all map blocks
        let mut next_range: Vec<i64> = seed.iter().cloned().collect(); //.iter().map(|x| *x).cloned().collect();

        // print seed number:
        // println!();
        println!("Current seed range: {:?}", &seed);

        // let mut next_seeds: Vec<Vec<i64>> = vec![vec![]]; //.iter().map(|x| *x).cloned().collect();
        let mut next_seeds: Vec<Vec<i64>> = Vec::from(Vec::from([])); //.iter().map(|x| *x).cloned().collect();
        let mut ranges_state: Vec<Vec<i64>> = Vec::from(Vec::from([]));
        ranges_state.push(next_range);

        for block in &map {
            let mut checked_complete: bool = false;
            // loop through all connection blocks:

            let mut next_seeds: Vec<Vec<i64>> = Vec::from(Vec::from([]));
            let mut next_range: Vec<Vec<i64>> = Vec::from(Vec::from([]));

            for range in block {
                println!("curr range of block line {:?}", &range);
                // skip the empty ones:
                if range.len() < 1 {
                    continue;
                }

                // initially there is just one -> then can be split into more!
                // loop through each range mapping:

                // process every range -> compare to limits:

                // for shift_range function:

                while ranges_state.len() > 0 {
                    // check each seed range in range:state against map_range:
                    let curr_seed_rng: Vec<i64> = ranges_state.pop().unwrap();
                    println!("curr_seed_rng {:?}", &curr_seed_rng);
                    println!("and the ranges_state after pop {:?}", &ranges_state);

                    // completely
                    if (curr_seed_rng[0] >= range[0]) & (curr_seed_rng[1] <= range[1]) {
                        let offset = curr_seed_rng[0] - range[0] - 1; // + 1 ?
                                                                      // the mapping is different than direct!
                        next_seeds.push(shift_range(range, &offset));

                        // now next_range is overwritten and can be given to next block (filter/mapping)

                        // if found one break off
                        checked_complete = true;
                        break;
                    } else if (curr_seed_rng[0] < range[0]) & (curr_seed_rng[1] >= range[0]) {
                        let offset = 0; // range[1] - &curr_seed_rng[1]; // + 1 ?
                                        // partial left
                        next_seeds.push(vec![curr_seed_rng[0], &range[0] - 1]);
                        next_seeds.push(shift_range(range, &offset));
                    } else if (curr_seed_rng[0] <= range[1]) & (curr_seed_rng[1] > range[1]) {
                        let offset = 0; // &curr_seed_rng[0] - &range[0]; // + 1 ?
                                        // partial right
                        next_seeds.push(vec![&range[1] + 1, curr_seed_rng[1]]);
                        next_seeds.push(shift_range(range, &offset));
                    } else {
                        // none of ranges match:
                        next_seeds.push(vec![curr_seed_rng[0], curr_seed_rng[1]]);
                    }

                    println!("next_seeds {:?}", &next_seeds);
                    // out of range -> map directly: means it stays the same!
                    if checked_complete {
                        //continue; //break; // break whole block!
                        break;
                    }
                }
            }
            // one block through -> overwrite ranges_state
            ranges_state = next_seeds
                .iter()
                .cloned()
                //.inspect(|x| println!("{:?}", x))
                .collect();

            // and clear next_seeds:
            next_seeds = Vec::from(Vec::from([]));

            // println!("{:?}", &block);
            // println!("next num: {}", &next_range);
        }

        // this seed is finished - collect its range:
        results.push(
            ranges_state
                .iter()
                .flatten()
                .map(|x| *x)
                .filter(|x| Some(x) != None)
                .collect::<Vec<_>>(),
        );
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // there was a i32 bit issue, parse() got None for overflow -> use i64!
    // println!(
    //     "\nThe result is: {:?}",
    //     &results //.iter().map(|x| x[0]).min().unwrap()
    // );

    println!(
        "\nThe result is: {:?}",
        results.sort() // .iter()
                       // .flatten()
                       // .filter(|x| *x != &vec![0, 0])
                       //.collect::<Vec<_>>() // results[1].iter().min().unwrap()
                       // results.iter().flatten().min().unwrap() // results[1].iter().min().unwrap()
    );

    // println!(
    //     "\nThe result is: {:?}",
    //     results.iter().map(|x| x[0]).min().unwrap()
    // );
}

// 2410909864 too high,
//
//    4064810 too low ,
