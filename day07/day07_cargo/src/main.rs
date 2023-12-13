/// Primary check: (ranks like cases)
/// find types: loop hand, save in Counter/HashMap, add to it if multiple occurences,
/// check max occurence -> match cases (still at most!):
/// case 5 occs: done
/// case 4 occs: done
/// case 3 occs: splits into: all other 1 occ, or one has 2 occs
/// case 2 occs: splits into (can not be more than 2 occs at this point!): another 2 occs, all other 1 occ
/// case 1 occ : done

/// Secondary check: (secondary rank like 'picture'=23456789TJQKA)
/// compare same cases
use counter::Counter;
use itertools::Itertools;
use std::fs;
use std::time::Instant;

pub fn read_txt(day: String) -> String {
    // read in puzzle input: dayXX.txt
    let suffix: &str = ".txt";

    let file_path: String = "./../../puzzle_inputs/day".to_owned() + &day + &suffix;

    let text: String = fs::read_to_string(file_path).expect("Could not open the text-file");

    return text;
}

#[derive(Debug)]
enum TypeOr<S, T> {
    Left(S),
    Right(T),
}

fn to_int(string: &str) -> TypeOr<String, i32> {
    // if error in parse to i32 -> give it as raw string
    let val = match string.parse::<i32>() {
        // return the Option<&str, _>
        Ok(s) => TypeOr::Right(s),
        Err(s) => TypeOr::Left(s.to_string()),
    };
    val
}

fn map_char(c: char) -> char {
    let new: char = match c {
        '2' => 'a',
        '3' => 'b',
        '4' => 'c',
        '5' => 'd',
        '6' => 'e',
        '7' => 'f',
        '8' => 'g',
        '9' => 'h',
        'T' => 'i',
        'J' => 'j',
        'Q' => 'k',
        'K' => 'l',
        'A' => 'm',
        _ => 'z',
    };
    new
}

fn convert_to_alpha(string: &str) -> String {
    // map c to new alphabet char
    let mut c_vec: Vec<char> = Vec::from([]);

    for c in string.chars() {
        c_vec.push(map_char(c));
    }

    c_vec.iter().join("")
}

fn main() {
    let day = String::from("07");
    // let day = String::from("07_test");

    // read in the text-file
    let txt: String = read_txt(day);

    let lines = txt.split("\n");

    let bids: Vec<_> = lines
        .clone()
        .map(|x| x.split(" ").collect::<Vec<_>>()[1])
        .collect::<Vec<_>>()
        .iter()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    let hands: Vec<_> = lines.map(|x| x.split(" ").collect::<Vec<_>>()[0]).collect();

    // now input has been parsed

    // start timer
    let now = Instant::now(); // mark time

    println!("hands {:?}; bids {:?}", &hands, &bids);

    // first decide the type: define rules, to check against;
    // match input lines against rules/types -> get a ranking/sorting
    // calc sum += rank * bid

    // loop through hand.len() and sort every same hands (by picture ranking)

    // Result: The result is: sum += rank * bid
    // Start:
    let mut types: Vec<_> = Vec::from([]);
    let mut most_commons: Vec<_> = Vec::from([]);

    for hand in hands.iter().cloned() {
        // save each hand in a Counter
        let mut hand_counter = hand.chars().collect::<Counter<_>>();

        // order the Counter, to get most common/ max
        // most_commons = hand_counter.most_common_ordered();
        most_commons = hand_counter.most_common_tiebreaker(|&a, &b| b.cmp(&a));
        println!("most commons {:?}", &most_commons);

        types.push(most_commons.clone());
    }

    // Primary check - first ranking:
    let mut ranks: Vec<(i32, i32)> = Vec::from([]);
    let mut types_sorted: Vec<Vec<(char, i32)>> = Vec::from([]);
    for (pos, ty) in types.iter().cloned().enumerate() {
        // match type:
        if ty[0].1 == 5 {
            // highest rank & continue
            ranks.push((pos as i32, 7));
        } else if ty[0].1 == 4 {
            ranks.push((pos as i32, 6));
        } else if ty[0].1 == 3 {
            // can split into next is 2 or 1:
            if ty[1].1 == 2 {
                // full house - 3 & 2 same
                ranks.push((pos as i32, 5));
            } else {
                // all other are 1 - 3 of kind
                ranks.push((pos as i32, 4));
            }
        } else if ty[0].1 == 2 {
            // pair: can split into - 2nd pair or others are 1
            if ty[1].1 == 2 {
                // 2 pairs
                ranks.push((pos as i32, 3));
            } else {
                // others are 1 - only pair
                ranks.push((pos as i32, 2));
            }
        } else {
            // first max occurence must be 1 - high card
            ranks.push((pos as i32, 1));
        }
    }

    // restructure the hands and bids, by ranks(pos, type):
    ranks.sort_by_key(|k| k.1);
    let mut new_hands: Vec<_> = Vec::from([]);
    let mut new_bids: Vec<_> = Vec::from([]);
    let mut new_types: Vec<Vec<_>> = Vec::from(Vec::from([]));
    for (new_pos, (pos, rank)) in ranks.clone().iter().enumerate() {
        // apply new order
        new_bids.push(bids[*pos as usize]);
        new_hands.push((convert_to_alpha(hands[*pos as usize]), new_pos));
        new_types.push(types[*pos as usize].clone());
    }

    println!(
        "new hands {:?}, new bids {:?} \nnew_types {:?}\n\n",
        &new_hands, &new_bids, &new_types
    );

    // now all types are matched and hands and bids reordered:

    // Secondary ordering - by 'picture'=23456789TJQKA:
    let ranks_counter = ranks.iter().cloned().map(|x| x.1).collect::<Counter<_>>();
    let mut final_ranks: Vec<(i32, i32)> = Vec::from([]);

    let mut pos: usize = 0;
    let mut occ: usize = 0;
    let mut prev_pos: usize = 0;
    let mut final_bids: Vec<_> = Vec::from([]);
    while pos < new_bids.len() {
        // ranks[pos].1 is the type -> check its occ in ranks_counter
        let ty = ranks[pos as usize].1;
        occ = *ranks_counter.get(&ty).unwrap();
        println!("occs: {}", &occ);

        let mut to_cmp_hands = Vec::from([]);

        if occ > 1 {
            // compare the next "occ" hands together
            to_cmp_hands = (&new_hands[pos..(pos + occ)]).to_vec();

            // sort alltogether
            // let mut new_poss = Vec::from([]);
            println!("to_comp_hands {:?}", &to_cmp_hands);

            // sort position with it as tuple()? -> how does number remain in it after sorting?
            to_cmp_hands.sort_by_key(|k| k.0.clone());
            println!("sorted strings - alphabetically {:?}", &to_cmp_hands);

            // reorder and save it:
            // let start_ind: usize = final_bids.len();
            // final_bids.push(
            //     new_bids[(prev_pos)..(pos as usize)]
            //         .into_iter()
            //         .collect::<Vec<_>>(),
            // );

            println!(
                "multiple occs: final_bids sliced for concat {:?}, from {} to {}",
                &final_bids, &prev_pos, &pos
            );

            for (new_pos, (hand, old_pos)) in to_cmp_hands.clone().iter().enumerate() {
                // apply new order
                final_bids.push(vec![&new_bids[*old_pos as usize]]);
            }
        } else if (occ == 1) & (pos < new_bids.len()) {
            // just one occurence

            final_bids.push(vec![&new_bids[pos]]);
            println!(
                "1 occ: final_bids sliced for concat {:?}, from {} to {}",
                &final_bids, &prev_pos, &pos
            );
        }
        // jump to next position, by current occurences
        prev_pos = pos;
        pos += occ;
    }

    println!(
        "rank counter {:?}",
        ranks.iter().map(|x| x.1).collect::<Counter<_>>()
    );

    println!("all hand counters {:?}", &types);
    println!("ranks {:?}", &ranks);
    //println!("all hand counters {:?}", &types_sorted);

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("\nThe final bids are: {:?}", &final_bids);

    let result = final_bids.into_iter().flatten().collect::<Vec<_>>();
    // println!("\nThe flattened final bids are: {:?}", &result);

    // The result:
    println!(
        "\nThe result is: {:?}",
        result
            .iter()
            .enumerate()
            .map(|(pos, val)| ((pos + 1) as i32) * *val)
            .sum::<i32>()
    );
}

// 246399406 too low,
// 246795406 - right
// 290320298 not right,
// 291035259 too high,
// 291469671 too high,
