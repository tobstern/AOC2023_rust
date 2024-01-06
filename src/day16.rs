//! --- Day 16: The Floor Will Be Lava ---
//!
//! Upon closer inspection, the contraption appears to be a flat, two-dimensional square grid containing empty space (.), mirrors (/ and \), and splitters (| and -).
//!
//! The beam enters in the top-left corner from the left and heading to the right. Then, its behavior depends on what it encounters as it moves:
//! If the beam encounters empty space (.), it continues in the same direction.
//! If the beam encounters a mirror (/ or \), the beam is reflected 90 degrees depending on the angle of the mirror. For instance, a rightward-moving beam that encounters a / mirror would continue upward in the mirror's column, while a rightward-moving beam that encounters a \ mirror would continue downward from the mirror's column.
//! If the beam encounters the pointy end of a splitter (| or -), the beam passes through the splitter as if the splitter were empty space. For instance, a rightward-moving beam that encounters a - splitter would continue in the same direction.
//! If the beam encounters the flat side of a splitter (| or -), the beam is split into two beams going in each of the two directions the splitter's pointy ends are pointing. For instance, a rightward-moving beam that encounters a | splitter would split into two beams: one that continues upward from the splitter's column and one that continues downward from the splitter's column.
use ndarray::Array2;
use std::collections::HashSet;
use std::time::Instant;

// move the light
fn step(
    last: (usize, usize),
    curr: (usize, usize),
    map: &Array2<((usize, usize), char)>,
    row_len: i32,
    col_len: i32,
) -> (Vec<bool>, Vec<(usize, usize)>) {
    let grad = ((curr.0 - last.0) as i32, (curr.1 - last.1) as i32);

    // let l_ch = *map.get(&last).unwrap();
    let c_ch = map[curr].1;

    let mut new_poss: Vec<(usize, usize)> = vec![];
    // if curr.i as i32 + grad.0 < 0
    //     && curr.j as i32 + grad.1 < 0
    //     && curr.i as i32 + grad.0 < row_len
    //     && curr.j as i32 + grad.1 < col_len
    // {
    // get next position, from left to right
    if grad == (0, 1) && c_ch == '|' {
        new_poss.extend(vec![(curr.0 + 1, curr.1), (curr.0 - 1, curr.1)]);
    } else if grad == (0, 1) && c_ch == '-' {
        new_poss.push((curr.0, curr.1 + 1));
    } else if grad == (0, 1) && c_ch == '.' {
        new_poss.push((curr.0, curr.1 + 1));
    } else if grad == (0, 1) && c_ch == '\\' {
        new_poss.push((curr.0 + 1, curr.1));
    } else if grad == (0, 1) && c_ch == '/' {
        new_poss.push((curr.0 - 1, curr.1));
    }

    // get next position, from right to left
    if grad == (0, -1) && c_ch == '|' {
        new_poss.extend(vec![(curr.0 + 1, curr.1), (curr.0 - 1, curr.1)]);
    } else if grad == (0, -1) && c_ch == '-' {
        new_poss.push((curr.0, curr.1 - 1));
    } else if grad == (0, -1) && c_ch == '.' {
        new_poss.push((curr.0, curr.1 - 1));
    } else if grad == (0, -1) && c_ch == '\\' {
        new_poss.push((curr.0 - 1, curr.1));
    } else if grad == (0, -1) && c_ch == '/' {
        new_poss.push((curr.0 + 1, curr.1));
    }

    // get next position, from top to bottom
    if grad == (1, 0) && c_ch == '|' {
        new_poss.push((curr.0 + 1, curr.1));
    } else if grad == (1, 0) && c_ch == '-' {
        new_poss.extend(vec![(curr.0, curr.1 + 1), (curr.0, curr.1 - 1)]);
    } else if grad == (1, 0) && c_ch == '.' {
        new_poss.push((curr.0 + 1, curr.1));
    } else if grad == (1, 0) && c_ch == '\\' {
        new_poss.push((curr.0, curr.1 + 1));
    } else if grad == (1, 0) && c_ch == '/' {
        new_poss.push((curr.0, curr.1 - 1));
    }

    // get next position, from bottom to top
    if grad == (-1, 0) && c_ch == '|' {
        new_poss.push((curr.0 - 1, curr.1));
    } else if grad == (-1, 0) && c_ch == '-' {
        new_poss.extend(vec![(curr.0, curr.1 + 1), (curr.0, curr.1 - 1)]);
    } else if grad == (-1, 0) && c_ch == '.' {
        new_poss.push((curr.0 - 1, curr.1));
    } else if grad == (-1, 0) && c_ch == '\\' {
        new_poss.push((curr.0, curr.1 - 1));
    } else if grad == (-1, 0) && c_ch == '/' {
        new_poss.push((curr.0, curr.1 + 1));
    }

    // filter out the positions that are out of bound
    let mut is_out_of_bound = Vec::new();
    let max = usize::MAX;
    for pos in new_poss.clone() {
        if pos.0 >= row_len as usize || pos.1 >= col_len as usize || pos.0 < max || pos.1 < max {
            is_out_of_bound.push(true);
        }
    }

    new_poss = new_poss
        .into_iter()
        .filter(|pos| {
            pos.0 < max && pos.1 < max && pos.0 < row_len as usize && pos.1 < col_len as usize
        })
        .collect();

    (is_out_of_bound, new_poss)
}

fn move_light(
    mut curr: (usize, usize),
    next_poss: &(Vec<bool>, Vec<(usize, usize)>),
    map: &Array2<((usize, usize), char)>,
    row_len: i32,
    col_len: i32,
    mut found: bool,
    mut states: &mut HashSet<((usize, usize), (usize, usize))>,
) -> bool {
    // loop through the map, according to rules for each tile; from last to curr
    // let mut states: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    // let mut last_states = states.clone();
    // let mut new_states = HashSet::new();

    // let next_poss: Vec<(usize, usize)> = Vec::new();

    let last = curr;
    'moving_light: for pos in &next_poss.1 {
        // 'moving_light: while next_poss.1.len() > 0 {

        println!(
            "\n\nnext_poss: {:?}, with curr_pos: {:?}",
            &next_poss.1, &pos
        );

        if pos == &curr {
            println!("last == curr");
            // continue 'moving_light;
        }
        // let pos = next_poss.1.pop().unwrap();
        println!("last (before): {:?}, curr (before): {:?}", &last, &curr);
        curr = *pos;

        // save the state
        // last_states = states.clone();
        states.insert((last, curr));

        println!("last: {:?}, curr: {:?}", &last, &curr);

        let (oob, nexts) = step(last, curr, &map, row_len, col_len);
        println!("oob: {:?}, nexts: {:?}", &oob, &nexts);

        // if oob {
        //     println!("out of bound!");
        //     // next_poss.extend(curr);
        //     continue 'moving_light;
        // }

        if nexts.len() == 0 {
            // next_poss.extend(curr);
            println!("no next pos(s)!\n");
            // path ended abruptly
            // states.insert((last, curr));
            continue 'moving_light;
        }
        // next_poss.extend(nexts);

        // check the state
        // if states.contains(&(curr, nexts[0])) {
        //     return states;
        // }
        // consider only checking all nexts...
        if states.contains(&(curr, nexts[0])) {
            found = true;
            // panic!("All are energized!");
            println!("\n\n-------------------All are energized!-------------------\n\n");
            return found;
        }

        // call itself recursively to move the light, when it reached a boundary of map
        // move the light
        found = move_light(
            curr,
            &(vec![false; nexts.len()], nexts),
            &map,
            row_len,
            col_len,
            found,
            &mut states,
        );
    }

    if found {
        return found;
    }

    found
}

pub fn parse_input(input: &String) -> Array2<((usize, usize), char)> {
    let lines: Vec<_> = input.lines().collect();
    let row_len = input.lines().count();
    let col_len = input.lines().next().unwrap().len();

    Array2::from_shape_fn((row_len, col_len), |(i, j)| {
        ((i, j), lines[i].chars().nth(j).unwrap())
    })
}

#[allow(unused)]
pub fn part1(input: String) {
    let row_len = input.lines().count();
    let col_len = input.lines().next().unwrap().len();

    // start timer
    let now = Instant::now(); // mark time

    let map = parse_input(&input);
    // println!("map: {:?}", &map);

    // start at the top left corner and head right at first
    let mut curr: (usize, usize) = (0, 1);
    let mut last: (usize, usize) = (0, 0);

    let mut new_poss = step(last, curr, &map, row_len as i32, col_len as i32);
    println!("new_poss: {:?}", &new_poss);

    // save state: last & curr
    let mut states = HashSet::new();
    let initial_state: ((usize, usize), (usize, usize)) = (last, curr);
    states.insert(initial_state);
    // let mut states: Vec<((usize, usize), (usize, usize))> = vec![(last, curr)];

    // move the light
    // let mut new_states: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    let mut found = false;
    found = move_light(
        curr,
        &new_poss,
        &map,
        row_len as i32,
        col_len as i32,
        found,
        &mut states,
    );

    // states.extend(new_states.to_owned());
    // states = new_states;

    println!("states: {:?}", &states);
    // let sum = states.len();
    // let sum = states
    //     .iter()
    //     .map(|(left, right)| vec![(left.i, left.j), (right.i, right.j)])
    //     .flatten()
    //     .collect::<HashSet<_>>();

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // for debugging pretty print the energized tiles in/with the map
    let mut print_map: Vec<Vec<char>> = input
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<_>>();
    // println!("print-map: {:?}", &print_map);

    for (i, j) in states
        .iter()
        .map(|(left, right)| vec![(left.0, left.1), (right.0, right.1)])
        .flatten()
    {
        print_map[i][j] = '#';
    }

    let bin_map = print_map
        .iter()
        .map(|line| {
            line.iter()
                .map(|ch| if ch == &'#' { 1 } else { 0 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // change all other symbols to '.' that are not '#'
    for i in 0..row_len {
        for j in 0..col_len {
            if print_map[i][j] != '#' {
                print_map[i][j] = '.';
            }
        }
    }

    let sum = bin_map.iter().flatten().sum::<usize>();
    println!("\nPart1 result is: {:?}", sum);

    println!("energized map: ");
    for line in print_map.iter() {
        println!("{:?}", line.iter().collect::<String>());
    }
}

#[allow(unused)]
pub fn part2(input: String) {
    let lines = input.split("\n");

    let line_vec: Vec<&str> = lines.collect();

    // start timer
    let now = Instant::now(); // mark time

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // println!("\nPart2 result is: {:?}", sum);
}

// 6975 too low
//
