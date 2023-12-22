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
use std::time::Instant;

// fn return_adjacents() ->
pub fn return_adjacent_posis(
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

pub fn part1(input: String) {
    let lines = input.split("\n");

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
            path_counts.extend(
                res_path
                    .iter()
                    .cloned()
                    .map(|x| x.len() / 2)
                    .collect::<Vec<_>>(),
            );
            // println!("Result path: {:?}", &res_path);
            println!(
                "maximum path length {:?} and path count is {}, with {:?}",
                &path_counts.iter().max(),
                &path_counts.len(),
                &path_counts
            );

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

    println!(
        "\nThe result is: {:?}",
        path_counts.iter().max().expect("No value found!")
    );
}

pub fn part2(input: String) {
    let lines = input.split("\n");

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
    let mut winding_nums: HashMap<(usize, usize), i32> = HashMap::new();

    let mut start_pos: (usize, usize) = (0, 0);
    for (i, line) in pipes.iter().cloned().enumerate() {
        for (j, tile) in line.iter().enumerate() {
            map.insert((i, j), *tile);

            // initialize all with 0
            winding_nums.insert((i, j), 0);

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
            path_counts.extend(
                res_path
                    .iter()
                    .cloned()
                    .map(|x| x.len() / 2)
                    .collect::<Vec<_>>(),
            );
            // println!("Result path: {:?}", &res_path);
            println!(
                "maximum path length {:?} and path count is {}, with {:?}",
                &path_counts.iter().max(),
                &path_counts.len(),
                &path_counts
            );

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

    let part1_res = path_counts.iter().max().expect("No value found!");
    println!("\nThe result is: {:?}", &part1_res);

    // part 2:
    // find every tile enclosed by the loop -> res = len()
    // use the Point in Polygon principle - Dan Sunday's winding number algorithm.
    // start timer
    let now = Instant::now(); // mark time

    // println!("res_path {:?}", &res_path);
    let res_path: Vec<(usize, usize)> = res_path
        .into_iter()
        .filter(|x| &x.len() / 2 == *part1_res)
        .collect::<Vec<_>>()[0]
        .clone();
    // println!("res_path {:?}", &res_path);

    // in res_path is now the loop.
    // let mut vert_gradient: i32 = 0;
    let mut last_pos: (usize, usize) = res_path[0].clone();
    let mut last_last_pos: (usize, usize) = res_path[0].clone();
    for (i, j) in &res_path {
        // update each tiles winding count -> for each horizontal/i position
        let vert_gradient = *i as i32 - last_last_pos.0 as i32;
        // let vert_gradient = *i as i32 - last_pos.0 as i32;

        // update all horizontal winding counts
        if vert_gradient != 0 {
            for col in *j..max_posis.1 {
                // for all columns - that are before the current column!
                if res_path.contains(&(last_pos.0, col)) {
                    continue;
                }

                // println!("vertical gradient {:?}, from column {} to {}", &vert_gradient, &last_last_pos.0, i);
                let curr_wind_num = winding_nums
                    .get(&(last_pos.0, col))
                    .expect("At this position is no winding number!");

                // println!("winding before {}, after {} ", curr_wind_num, curr_wind_num + &vert_gradient);
                winding_nums.insert((last_pos.0, col), curr_wind_num + &vert_gradient);
            }
        }

        // save position for next steps' gradient
        last_last_pos = last_pos;
        last_pos = (*i, *j);
    }

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // this probably should not be done with a HashMap -> takes about 1 min, but I am fine with that :-))

    // the part 2 result: every winding number != 0 should be inside polygon.
    // println!("winding numbers map {:?}", &winding_nums.iter().filter(|(_k, &v)| v != 0).collect::<Vec<_>>());
    println!(
        "The result of part 2 is {}",
        &winding_nums
            .iter()
            .filter(|(_k, &v)| v != 0)
            .collect::<Vec<_>>()
            .len()
    );
}
