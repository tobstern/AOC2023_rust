//! --- Day 16: The Floor Will Be Lava ---
//!
//! Upon closer inspection, the contraption appears to be a flat, two-dimensional square grid containing empty space (.), mirrors (/ and \), and splitters (| and -).
//!
//! The beam enters in the top-left corner from the left and heading to the right. Then, its behavior depends on what it encounters as it moves:
//! If the beam encounters empty space (.), it continues in the same direction.
//! If the beam encounters a mirror (/ or \), the beam is reflected 90 degrees depending on the angle of the mirror. For instance, a rightward-moving beam that encounters a / mirror would continue upward in the mirror's column, while a rightward-moving beam that encounters a \ mirror would continue downward from the mirror's column.
//! If the beam encounters the pointy end of a splitter (| or -), the beam passes through the splitter as if the splitter were empty space. For instance, a rightward-moving beam that encounters a - splitter would continue in the same direction.
//! If the beam encounters the flat side of a splitter (| or -), the beam is split into two beams going in each of the two directions the splitter's pointy ends are pointing. For instance, a rightward-moving beam that encounters a | splitter would split into two beams: one that continues upward from the splitter's column and one that continues downward from the splitter's column.
use image::{ImageBuffer, Rgb};
use ndarray::Array2;
use rayon::result;
use std::collections::HashSet;
use std::process::Command;
use std::time::Instant;

// move the light
fn step(
    last: (usize, usize),
    curr: (usize, usize),
    map: &Array2<((usize, usize), char)>,
    row_len: i32,
    col_len: i32,
    mut step_count: &mut usize,
    mut print_map: &mut Vec<Vec<char>>,
    video: bool,
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
    // get next position, to right
    if grad == (0, 1) {
        if c_ch == '|' {
            new_poss.extend(vec![(curr.0 + 1, curr.1), (curr.0 - 1, curr.1)]);
        } else if c_ch == '-' {
            new_poss.push((curr.0, curr.1 + 1));
        } else if c_ch == '.' {
            new_poss.push((curr.0, curr.1 + 1));
        } else if c_ch == '\\' {
            new_poss.push((curr.0 + 1, curr.1));
        } else if c_ch == '/' {
            new_poss.push((curr.0 - 1, curr.1));
        }
    }

    // get next position, to left
    if grad == (0, -1) {
        if c_ch == '|' {
            new_poss.extend(vec![(curr.0 + 1, curr.1), (curr.0 - 1, curr.1)]);
        } else if c_ch == '-' {
            new_poss.push((curr.0, curr.1 - 1));
        } else if c_ch == '.' {
            new_poss.push((curr.0, curr.1 - 1));
        } else if c_ch == '\\' {
            new_poss.push((curr.0 - 1, curr.1));
        } else if c_ch == '/' {
            new_poss.push((curr.0 + 1, curr.1));
        }
    }

    // get next position, to bottom
    if grad == (1, 0) {
        if c_ch == '|' {
            new_poss.push((curr.0 + 1, curr.1));
        } else if c_ch == '-' {
            new_poss.extend(vec![(curr.0, curr.1 + 1), (curr.0, curr.1 - 1)]);
        } else if c_ch == '.' {
            new_poss.push((curr.0 + 1, curr.1));
        } else if c_ch == '\\' {
            new_poss.push((curr.0, curr.1 + 1));
        } else if c_ch == '/' {
            new_poss.push((curr.0, curr.1 - 1));
        }
    }

    // get next position, to top
    if grad == (-1, 0) {
        if c_ch == '|' {
            new_poss.push((curr.0 - 1, curr.1));
        } else if c_ch == '-' {
            new_poss.extend(vec![(curr.0, curr.1 + 1), (curr.0, curr.1 - 1)]);
        } else if c_ch == '.' {
            new_poss.push((curr.0 - 1, curr.1));
        } else if c_ch == '\\' {
            new_poss.push((curr.0, curr.1 - 1));
        } else if c_ch == '/' {
            new_poss.push((curr.0, curr.1 + 1));
        }
    }

    // filter out the positions that are out of bound
    let mut is_out_of_bound = Vec::new();
    // let max = usize::MAX;
    for pos in new_poss.clone() {
        if pos.0 >= row_len as usize || pos.1 >= col_len as usize {
            is_out_of_bound.push(true);
        }
    }

    new_poss = new_poss
        .into_iter()
        .filter(|pos| pos.0 < row_len as usize && pos.1 < col_len as usize)
        .collect();

    // save a new image with the next position(s) of the light
    // step_count += 1;
    if video {
        make_pic(
            &new_poss,
            row_len as usize,
            col_len as usize,
            &mut step_count,
            &mut print_map,
        );
    }

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
    mut step_count: &mut usize,
    mut print_map: &mut Vec<Vec<char>>,
    video: bool,
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

        println!("last (before): {:?}, curr (before): {:?}", &last, &curr);
        curr = *pos;

        // save the state
        // last_states = states.clone();
        states.insert((last, curr));

        println!("last: {:?}, curr: {:?}", &last, &curr);

        let (oob, nexts) = step(
            last,
            curr,
            &map,
            row_len,
            col_len,
            &mut step_count,
            &mut print_map,
            video,
        );
        println!("oob: {:?}, nexts: {:?}", &oob, &nexts);

        if nexts.len() == 0 {
            // next_poss.extend(curr);
            println!("no next pos(s)!\n");
            // path ended abruptly
            // states.insert((curr, last));
            continue 'moving_light;
        }
        // next_poss.extend(nexts);

        if states.contains(&(curr, nexts[0])) {
            found = true;
            // panic!("All are energized!");
            println!("\n\n-------------------All are energized!-------------------\n\n");
            // return found;
            continue 'moving_light;
        }

        // call itself recursively to move the light, when it reached a boundary of map
        // move the light
        found = move_light(
            curr,
            &(vec![false; nexts.len()], nexts.clone()),
            &map,
            row_len,
            col_len,
            found,
            &mut states,
            &mut step_count,
            &mut print_map,
            video,
        );
    }

    // if found {
    //     return found;
    // }

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

fn make_pic(
    next_poss: &Vec<(usize, usize)>,
    row_len: usize,
    col_len: usize,
    step_count: &mut usize,
    print_map: &mut Vec<Vec<char>>,
) {
    let test = false;
    let scale_factor = if test { 100 } else { 5 }; // frames per second

    let row_len_scaled = row_len * scale_factor;
    let col_len_scaled = col_len * scale_factor;

    let mut img = ImageBuffer::new(row_len_scaled as u32, col_len_scaled as u32);
    // let mut img = ImageBuffer::new(col_len as u32, row_len as u32);

    for (i, j) in next_poss.iter() {
        // change the char of print map at this position to '#'
        print_map[*i][*j] = '#';
    }
    // now create all '#' from print_map as black pixels and all other symbols as white pixels
    // and oversample it (otherwise it is too small!)
    for (i, line) in print_map.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            let pixel = if next_poss.contains(&(i, j)) {
                // Rgb([255_u8, 255_u8, 0_u8]) // yellow/gold
                // Rgb([255_u8, 102_u8, 102_u8]) // red
                Rgb([0_u8, 255_u8, 0_u8]) // green
            } else if ch == &'#' {
                Rgb([255_u8, 215_u8, 0_u8])
            } else {
                Rgb([128_u8, 128_u8, 128_u8])
            };
            for x in 0..scale_factor {
                for y in 0..scale_factor {
                    img.put_pixel(
                        (j * scale_factor + x) as u32,
                        (i * scale_factor + y) as u32,
                        pixel,
                    );
                }
            }
        }
    }

    // Save the image
    *step_count += 1;
    img.save(format!("./images/day16/step{}.png", step_count))
        .unwrap();
}

fn create_video(num_steps: usize) {
    println!("Creating video from {} images...", num_steps);

    let output_dir = "./videos/"; // output directory
    let test = false;
    let fps = if test { "2" } else { "60" }; // frames per second
    let output = Command::new("ffmpeg")
        .arg("-y") // Automatically overwrite existing files
        .arg("-framerate")
        .arg(fps) // Set the frame rate
        .arg("-i")
        .arg("./images/day16/step%d.png") // Use the images as input
        .arg("-c:v")
        .arg("libx264rgb") // Set the video codec (losless)
        .arg("-r")
        .arg(fps) // Set the output frame rate
        .arg("-pix_fmt")
        .arg("rgb24") // Set the pixel format
        .arg("-vf")
        .arg("scale=iw:ih:flags=neighbor")
        // .arg("output.mp4") // Set the output file
        .arg(format!("{}output_day16_part1.mp4", output_dir)) // Set the output file with directory
        .output()
        .expect("Failed to execute FFmpeg");

    println!("FFmpeg output: {:?}", output);
}

#[allow(unused)]
pub fn part1(input: String) {
    // select video creation
    let video: bool = true;

    let row_len = input.lines().count();
    let col_len = input.lines().next().unwrap().len();

    // start timer
    let now = Instant::now(); // mark time

    let map = parse_input(&input);
    // println!("map: {:?}", &map);

    // for debugging pretty print the energized tiles in/with the map
    let mut print_map: Vec<Vec<char>> = input
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<_>>();
    // println!("print-map: {:?}", &print_map);

    // start at the top left corner and head right at first
    let mut curr: (usize, usize) = (0, 1);
    let mut last: (usize, usize) = (0, 0);

    // step count
    let mut step_count = 0;

    // include the first position of the light for the video!
    if video {
        make_pic(
            &vec![last, curr],
            row_len as usize,
            col_len as usize,
            &mut step_count,
            &mut print_map,
        );
    }

    let mut new_poss = step(
        last,
        curr,
        &map,
        row_len as i32,
        col_len as i32,
        &mut step_count,
        &mut print_map,
        video,
    );
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
        &mut step_count,
        &mut print_map,
        video,
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

    // create the video
    if video {
        create_video(step_count);

        // Remove the images
        let images_dir = "./images/day16/";
        std::fs::remove_dir_all(images_dir).expect("Failed to remove images directory");
        println!("Removed images directory: {} successfully!", images_dir);
        // create the removed folder again!
        std::fs::create_dir_all(images_dir).expect("Failed to create images directory");
    }

    // insert '#' for each pos from states into print_map
    let mut result = HashSet::new();
    for (left, right) in states.iter() {
        // if the found positions from states are not '#', change them to '#'
        if print_map[left.0][left.1] != '#' {
            print_map[left.0][left.1] = '#';
        }
        if print_map[right.0][right.1] != '#' {
            print_map[right.0][right.1] = '#';
        }

        // and insert them into a new HashSet to count them as result
        result.insert((left.0, left.1));
        result.insert((right.0, right.1));
    }
    // change all other symbols to '.' that are not '#'
    for i in 0..row_len {
        for j in 0..col_len {
            if print_map[i][j] != '#' {
                print_map[i][j] = '.';
            }
        }
    }

    println!("energized map: ");
    for line in print_map.iter() {
        println!("{:?}", line.iter().collect::<String>());
    }

    let sum = result.len();
    println!("\nPart1 result is: {:?}", sum);
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
