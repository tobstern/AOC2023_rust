//! --- Day 10: Pipe Maze ---
//! The pipes are arranged in a two-dimensional grid of tiles:
//!
//! | is a vertical pipe connecting north and south.
//! - is a horizontal pipe connecting east and west.
//! L is a 90-degree bend connecting north and east.
//! J is a 90-degree bend connecting north and west.
//! 7 is a 90-degree bend connecting south and west.
//! F is a 90-degree bend connecting south and east.
//! . is ground; there is no pipe in this tile.
//! S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
//!
//! Based on the acoustics of the animal's scurrying, you're confident the pipe that contains the animal is one large, continuous loop.

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

// fn return_adjacents() ->
fn return_adjacent_posis(
    (i, j): &(usize, usize),
    scheme: &HashMap<(usize, usize), char>,
    (max_i, max_j): (usize, usize),
    visited: &Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    // return Vec with all adjacent positions, if it is "connectable"

    // only consider NSWE, not diagonals here:
    let dirs: Vec<(i32, i32)> = Vec::from([
        (1, 0),
        // (1, 1),
        (0, 1),
        // (-1, 1),
        (-1, 0),
        // (-1, -1),
        (0, -1),
        // (1, -1),
    ]);

    let mut neighs: Vec<(usize, usize)> = Vec::new();

    for (m, n) in dirs {
        // test adjacents -> update neighs
        let curr_dir: (i32, i32) = (m, n);
        let neigh_pos: (i32, i32) = (*i as i32 + m, *j as i32 + n);

        // test if reached boundaries:
        if (neigh_pos.0 < 0)
            || (neigh_pos.1 < 0)
            || (neigh_pos.0 >= max_i as i32)
            || (neigh_pos.1 >= max_j as i32)
        {
            // if reached any boundary -> do not use it
            continue;
        }
        
        // println!("curr_direction: '{:?}' at pos: {:?}", &curr_dir, &neigh_pos);

        let neigh_pos = (neigh_pos.0 as usize, neigh_pos.1 as usize);
        let neigh: &char = scheme.get(&neigh_pos).unwrap_or_else(|| &'.');
        let curr_char: &char = scheme.get(&(*i, *j)).unwrap_or_else(|| &'.');

        // println!("from {} to neigh {} at pos: {:?}", &curr_char, &neigh, &neigh_pos);

        // match the direction to possible chars(connectors)
        let possible: Vec<char> = match (curr_dir, curr_char) {
            ((1, 0), '|') => vec!['|', 'J', 'L', 'S'],
            ((1, 0), '7') => vec!['|', 'J', 'L', 'S'],
            ((1, 0), 'F') => vec!['|', 'J', 'L', 'S'],
            ((1, 0), 'S') => vec!['|', 'J', 'L'],

            ((-1, 0), '|') => vec!['|', '7', 'F', 'S'],
            ((-1, 0), 'L') => vec!['|', '7', 'F', 'S'],
            ((-1, 0), 'J') => vec!['|', '7', 'F', 'S'],
            ((-1, 0), 'S') => vec!['|', 'F', '7'],

            ((0, 1), '-') => vec!['-', 'J', '7', 'S'],
            ((0, 1), 'F') => vec!['-', 'J', '7', 'S'],
            ((0, 1), 'L') => vec!['-', 'J', '7', 'S'],
            ((0, 1), 'S') => vec!['J', '7', '-'],

            ((0, -1), '-') => vec!['-', 'F', 'L', 'S'],
            ((0, -1), 'J') => vec!['-', 'F', 'L', 'S'],
            ((0, -1), '7') => vec!['-', 'F', 'L', 'S'],
            ((0, -1), 'S') => vec!['L', 'F', '-'],
            _ => vec![], //panic!("This is not a valid direction!"),
        };
        
        // check if S is neigh and return S
        // if &'S' == neigh && visited.len() > 2 {
        //     // found the Start
        //     neighs.push(neigh_pos);
        //     return neighs

        // }

        // now check if it is a valid neighbour (connectable!)
        // println!("possible? {}, not yet visited? {}", &possible.contains(neigh), &!visited.contains(&neigh_pos));
        if (possible.contains(neigh) && !visited.contains(&neigh_pos)) || &'S' == neigh {
            // println!("possible neigh position {:?}", &neigh_pos);
            // found connectable neigh - append it
            neighs.push(neigh_pos);
        }
    }
    // return cont
    neighs
}

fn main() {
    let day = String::from("10");
    // let day = String::from("10_test");

    // read in the text-file
    let txt: String = read_txt(day);

    let lines = txt.split("\n");

    let mut pipes: Vec<Vec<char>> = Vec::new();
    for line in lines {
        if line.len() > 0 {
            pipes.push(line.chars().collect());
        }
    }

    let max_posis: (usize, usize) = (pipes.len(), pipes[0].len());
    // println!("{:?}", &pipes);

    // start timer
    let now = Instant::now(); // mark time

    // Do Breadth First Search (BFS)
    // need a tree, and check for its neighbours -> HashMap,
    // neighbours are just usable, if connected! -> like '7' and 'L'; ''
    // find 'S' first -> end is when loop is closed, i.e. position already found
    // build HashMap<(i, j), 'tile'>
    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    let mut start_pos: (usize, usize) = (0, 0);
    for (i, line) in pipes.iter().cloned().enumerate() {
        for (j, tile) in line.iter().enumerate() {
            map.insert((i, j), *tile);

            // save starting position
            if tile == &'S' {
                start_pos = (i, j);
            }
        }
    }

    // println!("map {:?}, start_pos {:?}", map, start_pos);

    let mut queue: Vec<Vec<(usize, usize)>> = vec![vec![start_pos]];
    let mut res_path: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut path_counts: Vec<usize> = vec![];

    let mut c: usize = 0;
    while queue.len() > 0 {
        c += 1;
        let path: Vec<(usize, usize)> = queue.pop().unwrap();
        let tile_pos: (usize, usize) = path[&path.len() - 1];
        // println!(
        //     "path popped from queue {:?}, tile_pos {:?}",
        //     &path, &tile_pos
        // );
        // println!("queue after pop {:?}", &queue);

        // find adjacents in map @tile_pos
        let neighs: Vec<(usize, usize)> = return_adjacent_posis(&tile_pos, &map, max_posis, &path);

        // define stop condition:
        // stop when round-trip is complete, when reaching again 'S'
        // println!("current neighs before looping {:?}", &neighs);
        if tile_pos == start_pos && c > 1 {
            res_path.push(path.clone());
            path_counts.extend(res_path.iter().cloned().map(|x| x.len() / 2).collect::<Vec<_>>());
            // println!("Result path: {:?}", &res_path);
            println!("maximum path length {:?} and path count is {}, with {:?}", &path_counts.iter().max(), &path_counts.len(), &path_counts);

            // hardcode the break: stop if found enough paths/path_lengths -> the max is result!
            if path_counts.len() > 10 {
                break;
            }
        }

        // loop through adjacents
        for adj_pos in neighs {
            // println!("adjacent pos in neighbours {:?}", &adj_pos);

            if !path.contains(&adj_pos) || (adj_pos == start_pos && c > 2) {
                let mut new_path: Vec<(usize, usize)> = path.clone();
                new_path.push(adj_pos);
                queue.push(new_path.clone());
            }
        }

    }

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("\nThe result is: {:?}", path_counts.iter().max().expect("No value found!"));
}