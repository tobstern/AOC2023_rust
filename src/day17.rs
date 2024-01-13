//! --- Day 17: Clumsy Crucible ---
//! Directing the crucible from the lava pool to the machine parts factory, but not moving more than three consecutive blocks in the same direction, what is the least heat loss it can incur?
//! So, path search with weights and direction limitations.
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

// used from: https://doc.rust-lang.org/std/collections/binary_heap/index.html
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    cost: usize,
    position: (usize, usize),
    dir: (i32, i32),
    dir_cnt: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// function to cast tuple of numbers to tuple of i32 (that are losless convertable!)
#[allow(unused)]
pub fn cast_to_i32<T: TryInto<i32>>(
    tuple: (T, T),
) -> Result<(i32, i32), <T as TryInto<i32>>::Error> {
    let (i, j) = tuple;
    Ok((i.try_into()?, j.try_into()?))
}

// function to cast tuple of numbers to tuple of usize (that are losless convertable!)
#[allow(unused)]
pub fn cast_to_usize<T: TryInto<usize>>(
    tuple: (T, T),
) -> Result<(usize, usize), <T as TryInto<usize>>::Error> {
    let (i, j) = tuple;
    Ok((i.try_into()?, j.try_into()?))
}

fn dijkstra(
    city: &HashMap<(usize, usize), usize>,
    start: (usize, usize),
    end: (usize, usize),
) -> usize {
    // find the shortest and most efiicient (heat loss, etc...) path from start to end
    // dist[node] = current shortest distance from `start` to `node`

    let mut heap = BinaryHeap::new();
    // let mut seen = Vec::new();

    // We're at `start`, with a zero cost
    // dist[&start] = 0;
    // let start_dist = dist.get_mut(&start).unwrap();
    // *start_dist = 0;
    heap.push(State {
        cost: 0,
        position: start,
        dir: (0, 0),
        dir_cnt: 0,
    });

    // let mut queue: Vec<(usize, usize)> = Vec::new();
    // queue.push(start);
    let mut seen = HashSet::new();
    // seen.push((last, start));

    // let mut visited: Vec<(usize, usize)> = Vec::new();
    // let mut straight_count = 0;

    while let Some(State {
        cost,
        position,
        dir,
        dir_cnt,
    }) = heap.pop()
    {
        // println!("position: {:?} cost: {:?}", &position, &cost);

        if position == end {
            // println!("seen: {:?}", &seen);
            return cost;
        }

        if seen.contains(&(cost, position, dir, dir_cnt)) {
            continue;
        }

        seen.insert((cost, position, dir, dir_cnt));

        // let mut neighs = Vec::new();
        let (i, j) = position;
        // get row and col max of keys
        let rows = city.keys().map(|(i, _)| i).max().unwrap() + 1;
        let cols = city.keys().map(|(_, j)| j).max().unwrap() + 1;

        // let grad = (i as i32 - last_node.0 as i32, j as i32 - last_node.1 as i32);

        // the next neighbor can only be turn left, turn right, or go straight
        // never go straight longer than 3 times, use a counter for that
        // filterout oob positions in this process

        if dir_cnt < 3 && dir != (0, 0) {
            // go straight
            let (ni, nj) = (i as i32 + dir.0, j as i32 + dir.1);

            if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                // neighs.push((ni as usize, nj as usize));

                let next = State {
                    cost: cost + city[&(ni as usize, nj as usize)] as usize,
                    position: (ni as usize, nj as usize),
                    dir,
                    dir_cnt: dir_cnt + 1,
                };

                // If so, add it to the frontier and continue
                heap.push(next);
            }
        }

        // try turning in all of directions:
        let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        for (di, dj) in dirs {
            // if the nextdirection is 180 ° turn, then skip
            if (-dir.0, -dir.1) != (di, dj) && dir != (di, dj) {
                // check if the next position is oob
                let (ni, nj) = (i as i32 + di, j as i32 + dj);

                if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                    // now get next positions
                    // neighs.push((ni as usize, nj as usize));
                    let next = State {
                        cost: cost + city[&(ni as usize, nj as usize)] as usize,
                        position: (ni as usize, nj as usize),
                        dir: (di, dj),
                        dir_cnt: 1,
                    };

                    // If so, add it to the frontier and continue
                    heap.push(next);
                }
            }
        }
    }

    unreachable!() // this should never happen
}

#[allow(unused)]
pub fn part1(input: String) {
    let mut city = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            city.insert(
                (i, j),
                ch.to_digit(10).expect("this number is invalid!") as usize,
            );
            // println!("{:?}", line);
        }
    }
    // println!("{:?}", city);

    // start timer
    let now = Instant::now(); // mark time

    // do the Dijkstra-Algorithm

    // get the start and end positions
    let start = (0, 0);

    // get row and col max of keys
    let rows = city.keys().map(|(i, _)| i).max().unwrap() + 1;
    let cols = city.keys().map(|(_, j)| j).max().unwrap() + 1;
    let end = (rows - 1, cols - 1);

    // get the shortest path
    let path_weight = dijkstra(&city, start, end);

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("\nPart1 result is: {:?}", path_weight);
}

fn dijkstra_p2(
    city: &HashMap<(usize, usize), usize>,
    start: (usize, usize),
    end: (usize, usize),
) -> usize {
    // find the shortest and most efiicient (heat loss, etc...) path from start to end
    // dist[node] = current shortest distance from `start` to `node`

    let mut heap = BinaryHeap::new();
    // let mut seen = Vec::new();

    // We're at `start`, with a zero cost
    // dist[&start] = 0;
    // let start_dist = dist.get_mut(&start).unwrap();
    // *start_dist = 0;
    heap.push(State {
        cost: 0,
        position: start,
        dir: (0, 0),
        dir_cnt: 0,
    });

    // let mut queue: Vec<(usize, usize)> = Vec::new();
    // queue.push(start);
    let mut seen = HashSet::new();
    // seen.push((last, start));

    // let mut visited: Vec<(usize, usize)> = Vec::new();
    // let mut straight_count = 0;

    while let Some(State {
        cost,
        position,
        dir,
        dir_cnt,
    }) = heap.pop()
    {
        // println!("position: {:?} cost: {:?}", &position, &cost);

        if position == end && dir_cnt >= 4 {
            // println!("seen: {:?}", &seen);
            return cost;
        }

        if seen.contains(&(cost, position, dir, dir_cnt)) {
            continue;
        }

        seen.insert((cost, position, dir, dir_cnt));

        // let mut neighs = Vec::new();
        let (i, j) = position;
        // get row and col max of keys
        let rows = city.keys().map(|(i, _)| i).max().unwrap() + 1;
        let cols = city.keys().map(|(_, j)| j).max().unwrap() + 1;

        // let grad = (i as i32 - last_node.0 as i32, j as i32 - last_node.1 as i32);

        // the next neighbor can only be turn left, turn right, or go straight
        // never go straight longer than 3 times, use a counter for that
        // filterout oob positions in this process

        // for part 2 a max of ten straight moves is allowed
        if dir_cnt < 10 && dir != (0, 0) {
            // go straight
            let (ni, nj) = (i as i32 + dir.0, j as i32 + dir.1);

            if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                // neighs.push((ni as usize, nj as usize));

                let next = State {
                    cost: cost + city[&(ni as usize, nj as usize)] as usize,
                    position: (ni as usize, nj as usize),
                    dir,
                    dir_cnt: dir_cnt + 1,
                };

                // If so, add it to the frontier and continue
                heap.push(next);
            }
        }

        // try turning after 4 straight moves
        if dir_cnt >= 4 || dir == (0, 0) {
            // try turning in all of directions:
            let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
            for (di, dj) in dirs {
                // if the nextdirection is 180 ° turn, then skip
                if (-dir.0, -dir.1) != (di, dj) && dir != (di, dj) {
                    // check if the next position is oob
                    let (ni, nj) = (i as i32 + di, j as i32 + dj);

                    if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                        // now get next positions
                        // neighs.push((ni as usize, nj as usize));
                        let next = State {
                            cost: cost + city[&(ni as usize, nj as usize)] as usize,
                            position: (ni as usize, nj as usize),
                            dir: (di, dj),
                            dir_cnt: 1,
                        };

                        // If so, add it to the frontier and continue
                        heap.push(next);
                    }
                }
            }
        }
    }

    unreachable!() // this should never happen
}
#[allow(unused)]
pub fn part2(input: String) {
    let mut city = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            city.insert(
                (i, j),
                ch.to_digit(10).expect("this number is invalid!") as usize,
            );
            // println!("{:?}", line);
        }
    }
    // println!("{:?}", city);

    // start timer
    let now = Instant::now(); // mark time

    // do the Dijkstra-Algorithm

    // get the start and end positions
    let start = (0, 0);

    // get row and col max of keys
    let rows = city.keys().map(|(i, _)| i).max().unwrap() + 1;
    let cols = city.keys().map(|(_, j)| j).max().unwrap() + 1;
    let end = (rows - 1, cols - 1);

    // get the shortest path
    let path_weight = dijkstra_p2(&city, start, end);

    // record timer
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("\nPart2 result is: {:?}", path_weight);
}

// 735 too low
// 749 too low,
//

// part1: needed  7160 s to finish...???
// part2: needed 27113 s to finish...???
