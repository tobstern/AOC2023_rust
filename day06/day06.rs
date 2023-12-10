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
    // start timer
    let now = Instant::now(); // mark time

    let day = String::from("06");
    // let day = String::from("06_test");

    // read in the text-file
    let txt: String = read_txt(day);

    let lines = txt.split("\n");

    let line_vec: Vec<Vec<i32>> = lines
        .map(|x| {
            x.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        // .inspect(|x: &_| println!("ele is {:?}", x))
        .collect();

    println!("lines {:?}", line_vec);

    // zip time and distance together:
    let temp_dists = line_vec[1].iter().copied();
    let tidi: Vec<(i32, i32)> = line_vec[0].iter().copied().zip(temp_dists).collect();
    println!("tidi zipped {:?}", &tidi);

    // input is parsed:
    let mut result: Vec<i32> = Vec::from([]);
    for (i, (time, dist)) in tidi.iter().enumerate() {
        // loop Times:
        println!("race: {}; Time: {}; dist {}", &i, &time, &dist);

        // try all possibilities reaching given distance (aim)
        // thus save all possibilities >= aim dist;
        // depending on hold time

        let mut farther: Vec<i32> = Vec::from([]);

        for velo in 0..=*time {
            let mut self_dist: i32 = 0;
            // check for resulting max distance
            self_dist += velo * (time - velo);

            // if farther
            // println!("self distance {:?}", &result);
            if self_dist > *dist {
                farther.push(self_dist);
            }
        }
        result.push(farther.len() as i32);
        println!("result {:?}", &result);
    }

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("\nThe result is: {:?}", result.iter().product::<i32>());
}
