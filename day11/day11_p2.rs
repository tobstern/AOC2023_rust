use std::fs;
use std::time::Instant;
use std::collections::HashMap;

pub fn read_txt(day: String) -> String {
    // read in puzzle input: dayXX.txt
    let suffix: &str = ".txt";

    let file_path: String = "puzzle_inputs/day".to_owned() + &day + &suffix;

    let text: String = fs::read_to_string(file_path).expect("Could not open the text-file");

    return text;
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn main() {
    let day = String::from("11");
    // let day = String::from("11_test");

    // read in the text-file
    let txt: String = read_txt(day);

    // find epty space lines and columns
    let all_lines: Vec<Vec<char>> = txt.lines().map(|x| x.chars().collect()).collect();
    let empty_lines: Vec<bool> = all_lines.iter().filter_map(|x| Some(!x.contains(&'#'))).collect::<Vec<_>>();
    
    // println!("transposed Vec<Vec> {:?}", &all_cols);
    // println!("empty lines {:?}", &empty_lines);

    // pretty print lines of vec
    // for line in &all_lines {
    //     println!("expanded lines {:?}", &line);

    // }

    // now the same for all columns:
    let all_cols: Vec<Vec<char>> = transpose(all_lines.clone());
    let empty_cols: Vec<bool> = all_cols.iter().filter_map(|x| Some(!x.contains(&'#'))).collect::<Vec<_>>();
    // println!("empty columns {:?}", &empty_cols);
    
    // pretty print lines of final expanded Vec:
    // for line in &all_lines {
    //     println!("expanded lines {:?}", &line);

    // }

    // parse it:
    let mut map: HashMap<(usize, usize), i32> = HashMap::new();
    let mut galaxies : HashMap<i32, (usize, usize)> = HashMap::new();
    let mut count: i32 =  0;
    for (i, line) in all_lines.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            // do expansion for all empty lines/columns - double them

            // save each position and the space/galaxy as i32
            // println!("pos {:?} character {}", (i, j), ch);
            if ch == &'.' {
                // save as 0s
                map.insert((i, j), 0);
            } else if ch == &'#' {
                // save as incremented number
                count += 1;
                map.insert((i, j), count);
                galaxies.insert(count, (i, j));
            } else {
                // WTF: should not be anything else out here in space...
                continue;
            }

        }
    }

    // println!("the space map {:?}", &map);

    // start timer
    let now = Instant::now(); // mark time

    // now calculate the distances -> length of vector
    // number of combinations: n items and in r items per group
    // let galaxy_num: i128 = (*map.values().max().expect("WTF, there is no maximum available???")).into();
    // let _combs: i128 = fac(galaxy_num.into()) / (2 * fac((galaxy_num - 2).into()));

    // println!("number of combinations {:?}, test fac {}", &combs, fac(3));
    // get each unique pair
    let combinations : Vec<_> = galaxies.iter()
        .map(|(g, _v)| galaxies.iter().map(move |(g2, _v2)| (g2.to_owned(), g.to_owned())))
        .flatten()
        .collect();
    
    let combinations = combinations.iter().filter(|(left, right)| left != right).collect::<Vec<_>>();

    let mut uniques: Vec<(i32, i32)> = Vec::new();
    for ele in &combinations {
        if !&uniques.contains(ele) && !&uniques.contains(&(ele.1, ele.0)) {
            // it is unique -> append
            uniques.push(**ele);
        }
    }

    // println!("all combinations {:?}", &combinations);
    println!("all uniques {:?} with len of {}", &uniques, &uniques.len());

    let mut sum: i128 = 0;
    for (left, right) in uniques {
        // println!("{:?}", (&(i, j), &event));
        let pos1 = galaxies.get(&left).expect("Hä...no galaxy, on the left???");
        let pos2 = galaxies.get(&right).expect("Hä...no galaxy, on the right???");


        let mut row_start_end: Vec<usize> = vec![pos1.0, pos2.0];
        let mut col_start_end: Vec<usize> = vec![pos1.1, pos2.1];
        row_start_end.sort();
        col_start_end.sort();

        for i in row_start_end[0]..=row_start_end[1] {
            if empty_lines[i] {
                sum += 999999;
            }
        }

        for j in col_start_end[0]..=col_start_end[1] {
            if empty_cols[j] {
                sum += 999999;
            }
        }

        // let distance = ((pos2.0 as i32 - pos1.0 as i32 + 1).pow(2) as f32 + (pos2.1 as i32 - pos1.1 as i32 + 1).pow(2) as f32).sqrt();
        let distance = (pos1.0 as i128 - pos2.0 as i128).abs() + (pos1.1 as i128 - pos2.1 as i128).abs();
        // println!("current distance {}, for {} to {}", &distance, &left, &right);

        sum += distance;


    }

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("\nThe result is: {:?}", sum);
}
