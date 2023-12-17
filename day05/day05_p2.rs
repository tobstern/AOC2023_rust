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

fn main() {
    let day = String::from("05");
    // let day = String::from("05_test");

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
                // .inspect(|x| println!("inspect {:?}", &x))
                .skip_while(|x| x.contains("map:"))
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

    // switch to ranges: convert to: [dest, src, range] -> [src_start, src_len, dest_start, dest_len]
    // stay with range_start, range_length -> as otherwise it will loose the length after matching...
    let mut seeds: Vec<Vec<i64>> = Vec::new();
    for (i, ele) in seeds_raw.iter().enumerate() {
        // make ranges, so convert: [seed_start, range] -> [seed_start, seed_len]
        // remark: ranges are including len!

        if i % 2 == 0 {
            seeds.push(vec![*ele, seeds_raw[i + 1]]);
        }
    }

    let mut map: Vec<_> = map[1..(map.len())].to_vec().clone();

    // stay with range_start, range_length:
    let mut is_substraction: Vec<Vec<bool>> = Vec::new();
    let map_iter: Vec<Vec<_>> = map.clone();
    for (i, block) in map_iter.iter().enumerate() {
        let mut temp_bools: Vec<bool> = Vec::new();
        for (j, line) in block.iter().enumerate() {
            // println!("curr_line {:?}", &line);
            if &block.len() < &1 || &line.len() < &1 {
                continue;
            }

            // save order of source and destination
            temp_bools.push(line[0] < line[1]);
            // make ranges, so convert: [dest, src, range] -> [src_start, src_len, dest_start, dest_len]
            // remark: ranges are including len!
            map[i][j] = vec![line[1], line[2], line[0], line[2]];
        }

        is_substraction.push(temp_bools);
    }

    // println!("substraction order check {:?}", &is_substraction);
    // println!("seeds: {:?}", &seeds);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("seeds {:?}", seeds);
    // println!("blocks {:?}", &map);

    let now = Instant::now();
    // look up each of the seeds (values) -> find its location number -> min() is result!

    for (block_num, block) in map.iter().enumerate() {
        // write mapped ranges to next seed ranges that need to be mapped
        let mut ranges_with_mapping: Vec<Vec<i64>> = Vec::new();
        for seed in seeds {
            // saved as [destination, source, range]
            // loop through all map blocks

            // println!("Current seed range: {:?}", &seed);

            let mut overlaps: Vec<Vec<i64>> = Vec::new();

            for (line_num, range) in block.iter().enumerate() {
                // println!("curr range of block line {:?}", &range);

                // loop through each range mapping:
                // process every range -> compare to limits:

                // check each seed if overlapping:
                // let seed: Vec<i64> = seed.clone();
                // println!("seed {:?}", &seed);

                // any overlap?
                if (seed[0] < range[0] + range[1]) && (seed[0] + seed[1] > range[0]) {
                    let overlap_start = seed[0].max(range[0]);
                    let overlap_len = (seed[0] + seed[1]).min(range[0] + range[1]) - overlap_start;

                    overlaps.push(vec![overlap_start, overlap_len]);

                    // translate/map overlap to new_range
                    // overlap can be left/right:
                    // println!("block_number {}, line_number {}", &block_num, &line_num);
                    let mapped_start = if is_substraction[block_num][line_num] {
                        // shift left
                        overlap_start - (range[0] - range[2]).abs()
                    } else {
                        // shift right
                        overlap_start + (range[0] - range[2]).abs()
                    };

                    let next_mapped_rng = vec![mapped_start, overlap_len];
                    // println!("In overlap: next mapped range {:?}", &next_mapped_rng);

                    ranges_with_mapping.push(next_mapped_rng);
                }

                // out of range -> map directly: means it stays the same!
            }

            // map unmatched ranges
            // overlaps.sort_by_key(|x| x[0]);
            overlaps.sort();

            // find each range that has not been matched yet -> no change, just save it
            let mut unmapped_start = seed[0];

            for ovlp_range in overlaps {
                if unmapped_start < ovlp_range[0] {
                    // unmapped_start is indeed the start
                    ranges_with_mapping.push(vec![unmapped_start, ovlp_range[0] - unmapped_start]);
                }
                unmapped_start = ovlp_range[0] + ovlp_range[1];
            }

            if unmapped_start < (seed[0] + seed[1]) {
                ranges_with_mapping.push(vec![unmapped_start, seed[0] + seed[1] - unmapped_start]);
            }

            // println!("{:?}", &block);
            // println!("next num: {}", &curr_seed);
        }
        // println!("overall: mapped ranges {:?}", &ranges_with_mapping);

        seeds = ranges_with_mapping;
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
        seeds.iter().map(|range| range[0]).min().unwrap()
    );
}

// 2410909864 too high,
//  289863851 not right
//
//    4064810 too low ,
