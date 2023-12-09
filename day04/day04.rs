use std::fs;

pub fn read_txt(day: String) -> String {
    // read in puzzle input: dayXX.txt
    let suffix: &str = ".txt";

    let file_path: String = "puzzle_inputs/day".to_owned() + &day + &suffix;

    let text: String = fs::read_to_string(file_path).expect("Could not open the text-file");

    return text;
}

fn main() {
    let day = String::from("04");
    // let day = String::from("04_test");

    // read in the text-file
    let txt: String = read_txt(day);

    let lines = txt.split("\n");

    let line_vec: Vec<&str> = lines.collect();

    let mut cards: Vec<Vec<String>> = vec![];
    let mut wins_str: Vec<Vec<&str>> = vec![];
    let mut whatihave_str: Vec<Vec<&str>> = vec![];

    // the result sum
    let mut sum: i32 = 0;

    // parse the input:
    for l in &line_vec {
        let temp = l
            .clone()
            .split(": ")
            .map(|x: &str| {
                x.split(" | ")
                    // .collect::<Vec<String>>()
                    // .iter()
                    .map(|x: &str| x.split(" ").collect::<Vec<_>>())
            })
            .collect::<Vec<_>>();

        // println!("{:?}", &temp);
        for new_l in temp {
            // println!("new line: {:?}", new_l.clone().collect::<Vec<_>>());
            let temp_vec: Vec<_> = new_l.clone().collect();
            // println!("temp_vec: {:?}", &temp_vec[0]);

            if temp_vec[0].contains(&"Card") {
                continue;
            } else {
                // save each vec
                wins_str.push(temp_vec[0].clone());
                whatihave_str.push(temp_vec[1].clone());
            }
        }
    }
    //
    println!("wins_str: {:?}", &wins_str);
    println!("whatihave_str: {:?}", &wins_str);

    // loop through count of inner vectors and check if the win numbers are in "whatihave"
    for (i, num_vec) in wins_str.iter().enumerate() {
        // loop through one set:
        let mut count: i32 = -1;

        for num_str in num_vec {
            // eliminate "":
            if num_str == &"" {
                continue;
            }
            // check
            if whatihave_str[i].contains(num_str) {
                // i have the winning number -> count up
                count += 1;
            } else {
                // no winning number in set
                continue;
            }
        }

        // final result accumulation
        if count >= 0 {
            sum += 2_i32.pow(count as u32);
        }
    }
    // result:
    println!("The result is: {}", sum);
}
