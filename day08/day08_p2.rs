use std::collections::HashMap;
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
    let day = String::from("08");
    // let day = String::from("08_test3");

    // read in the text-file
    let txt: String = read_txt(day);

    // as HashMap -> split @\n\n -> 1st is instr; 2nd is map
    // split map @\n then @' = ', then the pos1 @', '| read capital chars with regexp :)
    let lines = txt.split("\n\n");
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
                let Some((key, val_str)) = line.split_once(" = ") else {continue;};
                let Some((left_temp, right_temp)) = val_str.trim().split_once(", ") else {continue;};
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
    // part 2:
    // start at every node ending with A (@same time)
    // run until every found node of every chosen paths end with Z.
    let mut count: usize = 0;
    let mut curr_nodes: Vec<&str> = Vec::from([]);
    // let mut curr_instr: char = 'X';
    // find all starting nodes: "XXA"
    for node in map.keys() {
        println!("node {:?}", &node);
        if node.chars().collect::<Vec<_>>()[2] == 'A' {
            // it is a starting node
            curr_nodes.push(node);
        }
    }

    println!("curr/starting_nodes {:?}", &curr_nodes);

    // loop through all starting nodes, simultaneously,
    // stop when all next nodes are ending with 'Z'

    while !curr_nodes
        .iter()
        .all(|x| x.chars().collect::<Vec<_>>()[2] == 'Z')
    {
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
        count += 1;
        // println!("curr_nodes {:?}", &curr_nodes);
    }

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("\nThe result is: {:?}", count);
}
