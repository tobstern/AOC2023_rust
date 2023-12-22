use std::collections::HashMap;
use std::time::Instant;

pub fn gcd(a: i64, b: i64) -> i64 {
    // Function to return gcd of a and b
    // sort a and b
    // let mut c = a;
    // if a < b {
    //     // swap order
    //     a = b;
    //     b = c;
    // }

    if a == 0 {
        return b;
    }
    // println!("gcd of {:?} is: {}", &(a, b), &gcd(b, a % b));
    return gcd(b % a, a);
}

pub fn lcm(a: i64, b: i64) -> i64 {
    // least common multiple
    println!(
        "lcm of {:?} is: {}",
        &(a, b),
        (a.abs() * b.abs() / gcd(a, b)) as i64
    );
    return ((a.abs() / gcd(a, b)) as i64) * b.abs();
}

#[derive(Debug)]
pub struct Periods {
    count: usize,
    is_period: bool,
}

pub fn part1(input: String) {
    // as HashMap -> split @\n\n -> 1st is instr; 2nd is map
    // split map @\n then @' = ', then the pos1 @', '| read capital chars with regexp :)
    let lines = input.split("\n\n");
    let mut instr: Vec<char> = Vec::from(Vec::from([]));
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for (i, block) in lines.enumerate() {
        // first is instructions
        if i == 0 {
            instr = block.trim().chars().collect();
        }

        // second is HasMap
        if i == 1 {
            for line in block.lines() {
                let Some((key, val_str)) = line.split_once(" = ") else {
                    continue;
                };
                let Some((left_temp, right_temp)) = val_str.trim().split_once(", ") else {
                    continue;
                };
                let left = left_temp.split_once("(").unwrap().1;
                let right = right_temp.split_once(")").unwrap().0;
                // println!("key, val {}, val {}", &key, &val);

                map.insert(key, (left, right));
            }
        }
    }
    // check parsed data:
    println!("instr: {:?}", &instr);
    println!("map: {:?}", &map);
    // parsed successfully.

    // start timer
    let now = Instant::now(); // mark time
                              // start mapping and counting the steps to find node "ZZZ"
    let mut count: usize = 0;
    let mut curr_node: &str = "AAA";
    let mut curr_instr: char = instr[0];

    while curr_node != "ZZZ" {
        // get curr instruction - cyclic repetition
        curr_instr = instr[count % instr.len()];

        // get next node
        if curr_instr == 'L' {
            // left from HashMap values
            curr_node = map.get(&curr_node).unwrap().0;
        } else if curr_instr == 'R' {
            // right from HashMap values
            curr_node = map.get(&curr_node).unwrap().1;
        } else {
            // has been a None value?
            continue;
        }
        count += 1;
    }

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("\nThe result is: {:?}", count);
}

pub fn part2(input: String) {
    // as HashMap -> split @\n\n -> 1st is instr; 2nd is map
    // split map @\n then @' = ', then the pos1 @', '| read capital chars with regexp :)
    let lines = input.split("\n\n");
    let mut instr: Vec<char> = Vec::from(Vec::from([]));
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for (i, block) in lines.enumerate() {
        // first is instructions
        if i == 0 {
            instr = block.trim().chars().collect();
        }

        // second is HasMap
        if i == 1 {
            for line in block.lines() {
                let Some((key, val_str)) = line.split_once(" = ") else {
                    continue;
                };
                let Some((left_temp, right_temp)) = val_str.trim().split_once(", ") else {
                    continue;
                };
                let left = left_temp.split_once("(").unwrap().1;
                let right = right_temp.split_once(")").unwrap().0;
                // println!("key, val {}, val {}", &key, &val);

                map.insert(key, (left, right));
            }
        }
    }
    // check parsed data:
    // println!("instr: {:?}", &instr);
    // println!("map: {:?}", &map);
    // parsed successfully.

    // start timer
    let now = Instant::now(); // mark time

    // start mapping and counting the steps to find node "ZZZ"
    // part 2:
    // start at every node ending with A (@same time)
    // run until every found node of every chosen paths end with Z.
    let mut count: usize = 0;
    let mut curr_nodes: Vec<&str> = Vec::from([]);
    // let mut curr_instr: char = 'X';
    // find all starting nodes: "XXA"
    for node in map.keys() {
        // println!("node {:?}", &node);
        if node.chars().collect::<Vec<_>>()[2] == 'A' {
            // it is a starting node
            curr_nodes.push(node);
        }
    }

    println!("curr/starting_nodes {:?}", &curr_nodes);

    // loop through all starting nodes, simultaneously,
    // save count when next node is ending with 'Z'

    // consider using periodicity -> found cycle when detecting the 'Z'
    // then find smallest common multiple of its len each!
    let mut periods_counts: Vec<_> = Vec::from([]);

    // insert all starting nodes in separate Vecs
    for _node in &curr_nodes {
        // periods_check.push(vec![]);
        // periods_check.push(vec![node.to_string()]);
        periods_counts.push(Periods {
            count: 0,
            is_period: false,
        });
    }

    let mut all_lcms: Vec<_> = Vec::from([]);
    let mut found_lcm: bool = false;
    loop {
        // get curr instruction - cyclic repetition
        let curr_instr = instr[count % instr.len()];

        // get next nodes (for each previous nodes)
        let mut next_nodes: Vec<&str> = Vec::from([]);
        for node in &curr_nodes {
            if curr_instr == 'L' {
                // left from HashMap values
                next_nodes.push(map.get(node).unwrap().0);
            } else if curr_instr == 'R' {
                // right from HashMap values
                next_nodes.push(map.get(node).unwrap().1);
            } else {
                // has been a None value?
                continue;
            }
        }

        curr_nodes = next_nodes;

        // found next string -> go check periods:

        // check for periods/ for the 'Z':
        // let period: usize = 0;
        for (idx, node) in curr_nodes.iter().enumerate() {
            if !periods_counts[idx].is_period {
                // compare each element with next half of nodes_vec
                let is_period: bool = node.chars().collect::<Vec<_>>()[2] == 'Z';

                if is_period {
                    // set the period count if it not has been set already
                    periods_counts[idx].count = count + 1;
                    periods_counts[idx].is_period = true;

                    // panic!("Found a period {:?}", &periods_counts[idx]);
                }
            }
        }

        // if all periods found: find gcd!
        let all_found: bool = periods_counts.iter().all(|x| x.is_period == true);
        // println!(
        //     "periods count {:?}, all_found? {}",
        //     &periods_counts, &all_found
        // );

        if all_found {
            // calculate gcd and then lcm, then break of while loop
            // then, result = all_periods_counts.iter().map(|x| x.count).sum()
            let mut periods: Vec<i64> = periods_counts.iter().map(|x| x.count as i64).collect();

            println!("all found with counts of: {:?}", &periods);
            let mut res: i64 = periods.pop().unwrap();
            let mut res2: i64 = res;
            for ele in &periods {
                // apply lcm for each result and ele
                res = lcm(res, *ele);
                res2 = gcd(res2, *ele);
            }
            all_lcms.push(res);
            // all_lcms.push(periods.iter().product());
            println!("all lcms: {:?}", &all_lcms);
            println!("gcd: {:?}", &res2);

            found_lcm = true;
        }

        if found_lcm {
            break;
        }

        // println!("curr_nodes {:?}", &curr_nodes);
        count += 1;
        // println!("round: {}", count);
    }

    println!("Found lcm: {:?}", all_lcms);

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("\nThe result is: {:?}", all_lcms[0]);
}
