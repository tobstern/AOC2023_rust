use std::fs;
use std::time::Instant;

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

    // consider raw seeds for part 1 -> seeds_ranges for part 2:
    let p1: bool = true;
    let mut seeds: Vec<_> = map[0][0].iter().copied().collect();
    let mut all_seeds: Vec<Vec<_>> = Vec::new();

    if !p1 {
        // save ranges
        for (pos, num) in seeds.iter().enumerate() {
            println!("seed num pos: {}", &pos);
            // 0, 2, 4, ... is seed start
            // 1, 3, 5, ... is range count
            if (pos % 2) == 0 {
                let end = seeds[pos + 1] as u64;

                // okay, then loop it
                for int in (*num as u64)..=end {
                    all_seeds.push(vec![int as i64]);
                }
            }
        }
    }

    let map: Vec<_> = map[1..(map.len())]
        .into_iter()
        .map(|x| x.into_iter().cloned().collect::<Vec<_>>())
        .collect();

    if !p1 {
        seeds = all_seeds.iter().flatten().map(|x| *x).collect::<Vec<i64>>();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    // println!("{:?}", seeds);
    // println!("{:?}", &map);

    let now = Instant::now();
    // look up each of the seeds (values) -> find its location number -> min() is result!
    let mut results: Vec<i64> = Vec::from([]);

    for seed in seeds {
        // saved as [destination, source, range]
        // loop through all map blocks
        let mut next_num: i64 = seed;

        // print seed number:
        // println!();
        println!("Current seed num: {}", &seed);

        for block in &map {
            // loop through all connection blocks:

            for range in block {
                // skip the empty ones:
                if range.len() < 1 {
                    continue;
                }
                // loop through each range mapping:
                if (&next_num >= &range[1]) & (&next_num < &(range[1] + range[2])) {
                    // the mapping is different than direct!
                    next_num = &range[0] + &next_num - &range[1];

                    // if found one break off
                    break;

                    // is actually an else:
                } else if (&next_num < &range[1]) | (&next_num >= &(range[1] + range[2])) {
                    // if the current seed is: num >= r[1] + r[2] | num < r[1] -> skip
                    continue;
                }
                // out of range -> map directly: it stays the same!
            }
            // check to which number it has been converted:
            // println!("{:?}", &block);
            // println!("next num: {}", &next_num);
        }

        // this seed is finished - collect its location:
        results.push(next_num);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // there was a i32 bit issue, parse() got None for overflow -> use i64!
    println!("\nThe result is: {:?}", results.iter().min().unwrap());
}
