use std::fs;

// #[derive(Debug)]
pub fn read_txt(day: String) -> String {
    // read in puzzle input: dayXX.txt
    let suffix: &str = ".txt";

    let file_path: String = "puzzle_inputs/day".to_owned() + &day + &suffix;

    let text: String = fs::read_to_string(file_path).expect("Could not open the text-file");

    return text;
}

fn main() {
    // let day = String::from("01_test");
    let day = String::from("01");

    // read in the text-file
    let txt: String = read_txt(day);

    let lines = txt.split("\n");

    let line_vec: Vec<&str> = lines.collect();

    // save all numbers and pick 1st and last
    let mut cal_vals: Vec<i32> = vec![];

    for line in line_vec {
        let mut temp: Vec<String> = Vec::new();

        for cha in line.chars() {
            // println!("{}", cha);

            if cha.is_numeric() {
                // save into cal_vals
                let temp_str = String::from(cha);
                temp.push(temp_str)
            }
        }

        // remove numbers in between:
        if (temp.len() as i32) < 1 {
            // skip this
            continue;
        } else if (temp.len() as i32) == 1 {
            // double the number

            let s1: String = String::from(&temp[0]);

            print!("{:?}\t---\t", &s1);

            let s2: &str = &String::from(&temp[0]);

            cal_vals.push((s1 + s2).parse().unwrap());
        } else {
            // save first and last
            let first: String = String::from(&temp[0]);
            let last: String = String::from(&temp[temp.len() - 1]);

            print!("first: {:?}, last: {:?}\n", &first, &last);

            cal_vals.push((first + &last).parse().unwrap());
        }

        // add found numbers to new line
        // println!("{:?}", &temp)
    }
    let sum: i32 = cal_vals.iter().sum();

    println!("\nThe result is: {:?}", sum);
}
