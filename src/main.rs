// main.rs
use advent_of_code::read_txt;
mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Please provide the day and part as command line arguments. Example: cargo run -- day01 part1");
        return;
    }
    let day = &args[1];
    let part = &args[2];

    // provide the input to the day's function:
    // day can include "_test" to read the test input
    let input = read_txt(day.to_string());
    match (day.as_str(), part.as_str()) {
        ("1", "1") => day1::part1(input),
        ("1", "2") => day1::part2(input),
        ("2", "1") => day2::part1(input),
        ("2", "2") => day2::part2(input),
        ("3", "1") => day3::part1(input),
        ("3", "2") => day3::part2(input),
        ("4", "1") => day4::part1(input),
        ("4", "2") => day4::part2(input),
        ("5", "1") => day5::part1(input),
        ("5", "2") => day5::part2(input),
        ("6", "1") => day6::part1(input),
        ("6", "2") => day6::part2(input),
        ("7", "1") => day7::part1(input),
        ("7", "2") => day7::part2(input),
        ("8", "1") => day8::part1(input),
        ("8", "2") => day8::part2(input),
        ("9", "1") => day9::part1(input),
        ("9", "2") => day9::part2(input),
        ("10", "1") => day10::part1(input),
        ("10", "2") => day10::part2(input),
        ("11", "1") => day11::part1(input),
        ("11", "2") => day11::part2(input),
        ("12", "1") => day12::part1(input),
        ("12", "2") => day12::part2(input),
        _ => println!("Invalid day or part argument. Please provide a valid day such as 'day01' and a part such as '1'."),
    }
}
// ("day13", "1") => day13::1(),
// ("day13", "2") => day13::2(),
// ("day14", "1") => day14::1(),
// ("day14", "2") => day14::2(),
// ("day15", "1") => day15::1(),
// ("day15", "2") => day15::2(),
// ("day16", "1") => day16::1(),
// ("day16", "2") => day16::2(),
// ("day17", "1") => day17::1(),
// ("day17", "2") => day17::2(),
// ("day18", "1") => day18::1(),
// ("day18", "2") => day18::2(),
// ("day19", "1") => day19::1(),
// ("day19", "2") => day19::2(),
// ("day20", "1") => day20::1(),
// ("day20", "2") => day20::2(),
// ("day21", "1") => day21::1(),
// ("day21", "2") => day21::2(),
// ("day22", "1") => day22::1(),
// ("day22", "2") => day22::2(),
// ("day23", "1") => day23::1(),
// ("day23", "2") => day23::2(),
// ("day24", "1") => day24::1(),
// ("day24", "2") => day24::2(),
// ("day25", "1") => day25::1(),
// ("day25", "2") => day25::2(),
