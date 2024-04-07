// main.rs
use advent_of_code::read_txt;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Please provide the day and part as command line arguments. Example: cargo run [-r] 1 1");
        return;
    }
    let mut day: &str = &args[1];
    let part: &str = &args[2];

    // provide the input to the day's function:
    // day can include "_test" to read the test input
    let input = read_txt(day.to_string());

    if day.contains("test") {
        day = day.split("_test").next().unwrap();
        // println!("day is = {}", day);
    }
    match (day, part) {
        ("1", "1") => day01::part1(input),
        ("1", "2") => day01::part2(input),
        ("2", "1") => day02::part1(input),
        ("2", "2") => day02::part2(input),
        ("3", "1") => day03::part1(input),
        ("3", "2") => day03::part2(input),
        ("4", "1") => day04::part1(input),
        ("4", "2") => day04::part2(input),
        ("5", "1") => day05::part1(input),
        ("5", "2") => day05::part2(input),
        ("6", "1") => day06::part1(input),
        ("6", "2") => day06::part2(input),
        ("7", "1") => day07::part1(input),
        ("7", "2") => day07::part2(input),
        ("8", "1") => day08::part1(input),
        ("8", "2") => day08::part2(input),
        ("9", "1") => day09::part1(input),
        ("9", "2") => day09::part2(input),
        ("10", "1") => day10::part1(input),
        ("10", "2") => day10::part2(input),
        ("11", "1") => day11::part1(input),
        ("11", "2") => day11::part2(input),
        ("12", "1") => day12::part1(input),
        ("12", "2") => day12::part2(input),
        ("13", "1") => day13::part1(input),
        ("13", "2") => day13::part2(input),
        ("14", "1") => day14::part1(input),
        ("14", "2") => day14::part2(input),
        ("15", "1") => day15::part1(input),
        ("15", "2") => day15::part2(input),
        ("16", "1") => day16::part1(input),
        ("16", "2") => day16::part2(input),
        ("17", "1") => day17::part1(input),
        ("17", "2") => day17::part2(input),
        ("18", "1") => day18::part1(input),
        ("18", "2") => day18::part2(input),
        ("19", "1") => day19::part1(input),
        ("19", "2") => day19::part2(input),
        _ => println!("Invalid day or part argument. Please provide a valid day such as '1' or '1_test[X]' and a part such as '1'."),
    }
}

// ("20", "1") => day20::part1(input),
// ("20", "2") => day20::part2(input),
// ("21", "1") => day21::part1(input),
// ("21", "2") => day21::part2(input),
// ("22", "1") => day22::part1(input),
// ("22", "2") => day22::part2(input),
// ("23", "1") => day23::part1(input),
// ("23", "2") => day23::part2(input),
// ("24", "1") => day24::part1(input),
// ("24", "2") => day24::part2(input),
// ("25", "1") => day25::part1(input),
// ("25", "2") => day25::part2(input),
