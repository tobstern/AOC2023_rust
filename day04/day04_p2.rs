use std::collections::HashMap;
use std::fs;

// #[derive(Debug)]
pub fn read_txt(day: String) -> String {
    // read in puzzle input: dayXX.txt
    let suffix: &str = ".txt";

    let file_path: String = "puzzle_inputs/day".to_owned() + &day + &suffix;

    let text: String = fs::read_to_string(file_path).expect("Could not open the text-file");

    return text;
}

fn merge(mut owner: HashMap<i32, i32>, dummy: HashMap<i32, i32>) -> HashMap<i32, i32> {
    // merges 2 HashMap together -> mutates the 1st:
    for (key, val) in dummy {
        // Option: is key already there? -> add to its value:
        let owner_val: Option<_> = owner.get(&key);
        let ret_val = match owner_val {
            Some(x) => *x,
            None => 0,
        };

        owner.insert(key, ret_val + val);
    }
    owner
}

fn has_won(curr_wins: Vec<&str>, wins: Vec<Vec<&str>>, mine: Vec<Vec<&str>>, curr: i32) -> i32 {
    // loop through count of inner vectors and check if the win numbers are in "whatihave"
    // loop through one set:
    let mut count: i32 = 0;

    for num_str in curr_wins.iter() {
        // eliminate "":
        if num_str == &"" {
            continue;
        }
        // check -> curr - 1, because Cards start at 1
        if mine[(curr - 1) as usize].contains(num_str) {
            // i have the winning number -> count up
            count += 1;
        } else {
            // no winning number in set
            continue;
        }
    }
    count
}

fn main() {
    let day = String::from("04");
    // let day = String::from("04_test");

    // read in the text-file
    let txt: String = read_txt(day);

    let lines = txt.split("\n");

    let line_vec: Vec<&str> = lines.collect();

    let mut wins_str: Vec<Vec<&str>> = Vec::from([]);
    let mut whatihave_str: Vec<Vec<&str>> = Vec::from([]);

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
    // println!("wins_str: {:?}", &wins_str);
    // println!("whatihave_str: {:?}", &wins_str);

    // create HashMap with every win count per card:
    let mut wins_per_card: HashMap<i32, i32> = HashMap::new();
    let mut ind: usize = 0;
    for (i, win_nums) in wins_str.iter().enumerate() {
        // calc wins and add to HashMap

        // start counting from 1:
        ind = i + 1_usize;

        let win_cnt: i32 = has_won(
            win_nums.clone(),
            wins_str.clone(),
            whatihave_str.clone(),
            ind as i32,
        );

        // save it
        wins_per_card.insert(ind as i32, win_cnt);
    }

    // println!(
    //     "wins per card {:?}, and the sum {}",
    //     &wins_per_card,
    //     &wins_per_card.values().sum::<i32>()
    // );

    // scratch cards:
    // number of winning nums -> "copies" next num cards -> go in each card again -> check winning cards and "copy"
    // -> until no cards are won!
    let mut instances: HashMap<i32, i32> = HashMap::new();
    let mut ind: usize = 0;

    let mut current_wins: i32 = 0;
    let mut queue: Vec<i32> = Vec::from([]);

    for (i, curr_wins) in wins_str.iter().enumerate() {
        // start counting from 1:
        ind = i + 1_usize; // current card
        let orig_card: i32 = ind as i32;

        let mut curr_card: i32 = 0;

        current_wins = *wins_per_card.get(&(ind as i32)).unwrap();

        instances = merge(instances, HashMap::from([(orig_card, 1)]));
        // println!(
        //     "run {} instances {:?} sum {}",
        //     &i,
        //     &instances,
        //     &instances.values().sum::<i32>()
        // );

        // loop while wins are found: - while queue is not empty
        queue.push(orig_card); // current card
        while queue.len() > 0 {
            // loop each card in queue
            // -> to find its wins and put the new/copied cards into queue,
            // then go again through queue
            curr_card = queue.pop().unwrap();
            current_wins = *wins_per_card.get(&curr_card).unwrap();

            // if current_wins < 1 {
            //     // break while loop due to no wins:
            //     break;
            // }

            // instances = merge(instances, HashMap::from([(curr_card), (current_wins)]));

            // loop through every new instances (defined by current_wins)
            for offset in 1..=current_wins {
                // save card id
                let next_curr: i32 = curr_card + offset;

                // add next card to queue:
                queue.push(next_curr);

                instances = merge(instances, HashMap::from([(next_curr, 1)]));
                // let temp = instances.clone();
                // let ret_val: i32 = match temp.get(&next_curr) {
                //     Some(x) => *x,
                //     None => 0,
                // };

                // instances.insert(next_curr, ret_val + 1); // unwrap if not none else give 0!
            }
        }
    }

    // result:
    println!("The result is: {}", instances.values().sum::<i32>());
}
