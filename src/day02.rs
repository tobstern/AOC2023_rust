use std::collections::HashMap;

pub fn part1(input: String) {
    let lines = input.split("\n");

    let line_vec: Vec<&str> = lines.collect();

    // each line a Game,
    // split @ ": ",
    // and then @ "; " to get each Hand (subset):
    // and finally @ ", " to get each 'count color' (individual cubes)
    // do not forget to trim() the Strings!
    let mut game_count: i32 = 0;

    let mut sum: i32 = 0;

    // initialize the CubesCount for maximum count of cubes per color:
    let max_cubes: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for line in line_vec {
        let mut all_cubes: Vec<HashMap<&str, i32>> = vec![];

        game_count += 1;

        let set_parts: Vec<&str> = line.split(": ").collect(); //::<Vec<_>>()[1].trim();
        let set: Vec<&str> = set_parts[1].split("; ").collect();

        let mut subsets: Vec<Vec<Vec<&str>>> = vec![vec![vec![]]];

        // let mut subsets: Vec<_> = set.into_iter().map(|s| s.split(", ")).collect();
        // let mut subsets: Vec<&str> = set.iter().map(|x| x.split("; ").collect());
        for hand in set {
            // split @", " -> get each color
            // println!("hand: {:?}", &hand.split(", ").collect::<Vec<_>>());
            // temp vec for correct concat of each hand (separately)
            let mut temp: Vec<_> = vec![];

            for color in hand.split(", ") {
                //.collect::<Vec<_>>().iter()
                let inner: Vec<&str> = color.split_whitespace().collect::<Vec<_>>();
                // println!("hand: {:?}", &hand.split(", ").collect::<Vec<_>>());
                // println!("color: {:?}", color.split_whitespace().collect::<Vec<_>>());

                temp.push(inner);
            }

            subsets.push(temp);

            // println!("subsets: {:?}", &subsets);
            // from here all Strings are correctly separated and concatenated!
        }

        println!("subsets: {:?}", &subsets);
        // now start matching and adding together
        // create HashMap summing values for each line
        let mut cubes: HashMap<&str, i32> = HashMap::new();

        for subset in subsets {
            // if (subset[0].len() as i32) < 1 {
            //     continue;
            // } else {
            // match each color and save the number to it
            for (_, color) in subset.iter().enumerate() {
                if (color.len() as i32) < 2 {
                    continue;
                }
                // colors are @ position [1], numbers @ [0]:
                // println!(
                //     "color_string: {:?}, old_number {:?}, curr_number {:?}",
                //     color[1],
                //     cubes.get(color[1]).unwrap(),
                //     color[0].to_string().parse::<i32>().unwrap()
                // );

                cubes.insert(color[1], color[0].to_string().parse::<i32>().unwrap());

                println!("game: {}, cubes {:?}", game_count, cubes);

                // save color and number to
            }
            // push new cubes to all_cubes collection
            all_cubes.push(cubes.clone());

            println!("all_cubes: {:?}", all_cubes);
        }

        // this round is over -> the overall cubes needs to be checked for max counts!
        let mut valid: bool = true;
        for hand in &all_cubes {
            for (k, v) in hand {
                // println!("k: {:?}, v: {:?}, hand: {:?}", k, v, hand);
                // println!(
                //     "lhs: {:?}, rhs: {}",
                //     (*max_cubes.get(k).unwrap() as i32),
                //     *v
                // );

                if (*max_cubes.get(k).unwrap() as i32) < *v {
                    valid = false;
                    println!("This game is already false! -> valid? = {}", valid);
                    break;
                }
            }
        }

        // if valid still true, Game is good!
        if valid {
            sum += game_count;
        }
    }

    println!("\nThe result is: {:?}", sum);
}

pub fn part2(input: String) {
    let lines = input.split("\n");

    let line_vec: Vec<&str> = lines.collect();

    // each line a Game,
    // split @ ": ",
    // and then @ "; " to get each Hand (subset):
    // and finally @ ", " to get each 'count color' (individual cubes)
    // do not forget to trim() the Strings!

    let mut sum: i32 = 0;

    for line in line_vec {
        let mut all_cubes: Vec<HashMap<&str, i32>> = vec![];

        let set_parts: Vec<&str> = line.split(": ").collect(); //::<Vec<_>>()[1].trim();
        let set: Vec<&str> = set_parts[1].split("; ").collect();

        let mut subsets: Vec<Vec<Vec<&str>>> = vec![vec![vec![]]];

        // let mut subsets: Vec<_> = set.into_iter().map(|s| s.split(", ")).collect();
        // let mut subsets: Vec<&str> = set.iter().map(|x| x.split("; ").collect());
        for hand in set {
            // split @", " -> get each color
            // println!("hand: {:?}", &hand.split(", ").collect::<Vec<_>>());
            // temp vec for correct concat of each hand (separately)
            let mut temp: Vec<_> = vec![];

            for color in hand.split(", ") {
                //.collect::<Vec<_>>().iter()
                let inner: Vec<&str> = color.split_whitespace().collect::<Vec<_>>();
                // println!("hand: {:?}", &hand.split(", ").collect::<Vec<_>>());
                // println!("color: {:?}", color.split_whitespace().collect::<Vec<_>>());

                temp.push(inner);
            }

            subsets.push(temp);

            // println!("subsets: {:?}", &subsets);
            // from here all Strings are correctly separated and concatenated!
        }

        println!("subsets: {:?}", &subsets);
        // now start matching and adding together
        // create HashMap summing values for each line
        let mut cubes: HashMap<&str, i32> = HashMap::new();

        for subset in subsets {
            // match each color and save the number to it
            for (_, color) in subset.iter().enumerate() {
                if (color.len() as i32) < 2 {
                    continue;
                }
                // colors are @ position [1], numbers @ [0]:
                // println!(
                //     "color_string: {:?}, old_number {:?}, curr_number {:?}",
                //     color[1],
                //     cubes.get(color[1]).unwrap(),
                //     color[0].to_string().parse::<i32>().unwrap()
                // );

                cubes.insert(color[1], color[0].to_string().parse::<i32>().unwrap());

                // save color and number to
            }
            // push new cubes to all_cubes collection
            all_cubes.push(cubes.clone());

            println!("all_cubes: {:?}", all_cubes);
        }

        // this round is over -> the overall cubes needs to be checked for max counts (of subsets)
        // min number of necessary cubes -> max occurences!
        let mut occ: HashMap<&str, i32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        let mut prod: i32 = 1;

        for hand in &all_cubes {
            for (k, v) in hand {
                // println!("k: {:?}, v: {:?}, hand: {:?}", k, v, hand);
                // println!(
                //     "lhs: {:?}, rhs: {}",
                //     (*max_cubes.get(k).unwrap() as i32),
                //     *v
                // );

                if (*occ.get(k).unwrap() as i32) < *v {
                    occ.insert(k, *v);
                }
            }
        }
        // collected in occ highest count of all (3) colors
        // now multiply together and sum

        // println!("occ.values: {:?}", occ.values().map(|x| x * x));
        for ele in occ.values() {
            println!("ele: {}", &ele);
            prod *= ele;
        }

        sum += prod;
    }

    println!("\nThe result is: {:?}", sum);
}
