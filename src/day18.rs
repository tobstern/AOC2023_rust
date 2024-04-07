//! --- Day 18: Lavaduct Lagoon ---
//! dig from plan in input.
//! Dig circumference of lagoon.
use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;

#[allow(unused)]
pub fn part1(input: String) {
    let mut dig_plan = Vec::new();
    let re = Regex::new(r"(?<dir>[LDRU])\s+(?<count>[0-9]+)\s+\(#(?<color>[a-z0-9]+)\)").unwrap();
    for line in input.lines() {
        // println!("{}", line.split_whitespace().collect::<String>());
        let extracted_line = re
            .captures_iter(line)
            .map(|caps| {
                let dir = caps.name("dir").unwrap().as_str().chars().next().unwrap();
                let count = caps.name("count").unwrap().as_str().parse::<i32>().unwrap();
                let color = caps.name("color").unwrap().as_str();
                (dir, count, color)
            })
            .collect::<Vec<(char, i32, &str)>>();
        dig_plan.extend(extracted_line);
    }

    // println!("dig_plan = {:?}", &dig_plan);

    // start timer
    let now = Instant::now(); // mark time

    let mut lagoon: HashMap<(i32, i32), &str> = HashMap::new();

    // now dig - 1 time:
    let mut pos: (i32, i32) = (0, 0);
    let mut path = Vec::new();

    path.push(pos);
    let mut edges = vec![(0, 0)];
    let mut last_dir = 'R';

    let mut circum = 1;
    for (_i, (dir, count, color)) in dig_plan.iter().enumerate() {
        // now create map (HashMap)

        // if direction changes, it is an edge
        if dir != &last_dir {
            edges.push(pos);
            last_dir = *dir;
        }

        // loop the count, dig each 1m hole (1 tile per step)
        for _inc in 1..=*count {
            circum += 1;

            // to left
            if dir == &'L' {
                pos = (pos.0, pos.1 - 1);
                lagoon.insert(pos, *color);
            }
            // to right
            if dir == &'R' {
                pos = (pos.0, pos.1 + 1);
                lagoon.insert(pos, *color);
            }

            // up
            if dir == &'U' {
                pos = (pos.0 - 1, pos.1);
                lagoon.insert(pos, *color);
            }
            // down
            if dir == &'D' {
                pos = (pos.0 + 1, pos.1);
                lagoon.insert(pos, *color);
            }

            // update the path Vec for winding number calc
            path.push(pos);
        }
    }

    // test circumference
    println!("{:?}", circum);

    // build the map -for debugging:
    let mut map = Vec::new();
    let mut winding_nums: HashMap<(i32, i32), i32> = HashMap::new();

    let rmax = *path.iter().map(|(y, _x)| y).max().unwrap();
    let cmax = *path.iter().map(|(_y, x)| x).max().unwrap();
    let rmin = *path.iter().map(|(y, _x)| y).min().unwrap();
    let cmin = *path.iter().map(|(_y, x)| x).min().unwrap();
    // let (rmax, cmax) = *lagoon.keys().map(|pos| pos).max().unwrap();

    for i in rmin..=rmax {
        let mut temp = Vec::new();
        for j in cmin..=cmax {
            // initialize all winding numbers (with 0)

            // save pos
            let pos = (i, j);
            // if lagoon.keys().collect::<Vec<_>>().contains(&&pos) || pos == (0, 0) {
            if edges.contains(&pos) {
                // pos is an edge, add -> +
                temp.push('+');
            } else if path.contains(&pos) {
                // pos is in lagoon so add #
                temp.push('#');
            } else {
                // not included: '.'
                temp.push('.');
            }
        }

        map.push(temp);
    }

    // pretty print map:
    for (i, line) in map.iter().enumerate() {
        println!("{}", line.iter().collect::<String>());
    }

    // now find the area/tile count enclosed by an irregular polygon
    // calculate polygon area:

    // add the first to end again
    edges.push(edges[0]);

    // calc products from curr_x to next_y coordinate and sum
    // & calc products curr_y to next_x coord. and sum
    // then subtract and divide by 2:

    let mut tot_sum = 0.0;
    for (i, (y, x)) in edges.iter().enumerate() {
        if i >= edges.len() - 1 {
            break;
        }

        tot_sum += (x * edges[i + 1].0 - y * edges[i + 1].1) as f64;
    }

    tot_sum /= 2.0;

    println!("circumference {:?}", path.len());
    tot_sum += (path.len() as f64) / 2.0;

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("Part 1 result is: {:?}", tot_sum.round() as i32);
}

#[allow(unused)]
pub fn part2(input: String) {
    let mut hexas = Vec::new();
    let re = Regex::new(r"(?<dir>[LDRU])\s+(?<count>[0-9]+)\s+\(#(?<color>[a-z0-9]+)\)").unwrap();
    for line in input.lines() {
        // println!("{}", line.split_whitespace().collect::<String>());
        let extracted_line = re
            .captures_iter(line)
            .map(|caps| {
                let color = caps.name("color").unwrap().as_str();
                color
            })
            .collect::<Vec<&str>>();
        hexas.extend(extracted_line);
    }

    // println!("dig_plan = {:?}", &dig_plan);
    // last digit is direction
    let mut dig_plan = Vec::new();
    let mut all_dirs = HashMap::from([(0, 'R'), (1, 'D'), (2, 'L'), (3, 'U')]);
    for word in hexas.iter() {
        let dir_int = word
            .char_indices()
            .nth_back(0)
            .unwrap()
            .1
            .to_string()
            .parse::<usize>()
            .unwrap();
        let dir = all_dirs[&dir_int];

        let hex_str = {
            let mut chs = word.chars();
            chs.next_back();
            chs.as_str()
        };
        let count = i64::from_str_radix(hex_str, 16).ok().unwrap();

        // test instructions
        // println!("dir: {:?}, count: {:?}", &dir, &count);

        dig_plan.push((dir, count));
    }

    // start timer
    let now = Instant::now(); // mark time

    let mut pos: (i64, i64) = (0, 0);

    let mut edges = vec![(0, 0)];
    let mut last_dir = 'R';
    let mut circum = 1;
    for (_i, (dir, count)) in dig_plan.iter().enumerate() {
        // now create map (HashMap)

        // if direction changes, it is an edge
        if dir != &last_dir {
            // update circumference, with curr pos (edge) - last edge:
            // let last_edge = edges[edges.len() - 1];
            // circum += pos.0 - last_edge.0 + pos.1 - last_edge.1;

            edges.push(pos);
            last_dir = *dir;
        }

        // loop the count, dig each 1m hole (1 tile per step)
        for _inc in 1..=*count {
            // update circumference
            circum += 1;

            // to left
            if dir == &'L' {
                pos = (pos.0, pos.1 - 1);
            }
            // to right
            if dir == &'R' {
                pos = (pos.0, pos.1 + 1);
            }

            // up
            if dir == &'U' {
                pos = (pos.0 - 1, pos.1);
            }
            // down
            if dir == &'D' {
                pos = (pos.0 + 1, pos.1);
            }
        }
    }

    // now find the area/tile count enclosed by an irregular polygon
    // calculate polygon area:

    // add the first to end again
    edges.push(edges[0]);

    // calc products from curr_x to next_y coordinate and sum
    // & calc products curr_y to next_x coord. and sum
    // then subtract and divide by 2:

    let mut tot_sum = 0.0;
    for (i, (y, x)) in edges.iter().enumerate() {
        if i >= edges.len() - 1 {
            break;
        }

        tot_sum += (x * edges[i + 1].0 - y * edges[i + 1].1) as f64;
    }

    tot_sum /= 2.0;

    println!("circumference {:?}", &circum);
    tot_sum += (circum as f64) / 2.0;

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("\nPart2 result is: {:?}", tot_sum.round() as i64);
}
