use std::fs;
use std::io::prelude::*;
use std::path::Path;
// aoc_input.rs
/// This module provides a function to download the puzzle input for a given day
/// from the Advent of Code website. It uses the session cookie stored in the
/// `session_cookie.txt` file to authenticate the request. If the puzzle input
/// file for the day already exists, it does not download the file again but
/// reads the existing file.
///
/// The `read_txt` function takes a `String` representing the day and returns
/// the puzzle input as a `String`.
///
#[allow(unused)]
pub fn read_txt(day: String) -> String {
    // read in the test-file if it exists
    let is_test: bool = day.ends_with("_test");
    let mut filename: String = String::new();
    if is_test {
        filename = format!("day{}_test.txt", day);
        if Path::new(&filename).exists() {
            return fs::read_to_string(filename)
                .expect("Something went wrong reading the test file");
        }
    } else {
        // read in puzzle input: dayXX.txt
        filename = format!("./puzzle_inputs/day{}.txt", day);
        if !Path::new(&filename).exists() {
            let url = format!("https://adventofcode.com/2023/day/{}/input", day);
            let session_cookie =
                fs::read_to_string("session_cookie.txt").expect("Failed to read session cookie");
            let client = reqwest::blocking::Client::new();
            let response = client
                .get(&url)
                .header("Cookie", format!("session={}", session_cookie.trim()))
                .send();
            match response {
                Ok(res) => {
                    if res.status().is_success() {
                        let mut file = fs::File::create(&filename).expect("create file failed");
                        file.write_all(res.text().unwrap().as_bytes())
                            .expect("write failed");
                        println!("File downloaded");
                    } else {
                        println!(
                            "Failed to download file, HTTP response code: {}",
                            res.status()
                        );
                    }
                }
                Err(e) => println!("Failed to download file, error: {}", e),
            }
        }
    }

    fs::read_to_string(filename).expect("Something went wrong reading the file")
}
