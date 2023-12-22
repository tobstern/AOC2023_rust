# AOC2023_rust

This repository contains my solutions for the Advent of Code 2023, written in Rust.

## Project Structure

The project is structured as a Rust Cargo project. Each day's puzzle is solved in a separate Rust module named `dayXX`, where `XX` is the day number. Each module contains two functions, `part1` and `part2`, which solve parts 1 and 2 of the day's puzzle, respectively.

## Running the Code

To run the code for a specific day and part, use the `cargo run` command followed by the day and part number. For example, to run the solution for day 1, part 1, you would use the following command:

```bash
cargo run -- 1 1
```

## Input Data
The input data for each day's puzzle is read from a text file using a function from the lib.rs. The text files are named according to the day number and are located in the input directory.
For the full input, it will be attempted to download it, thus save your __Session Cookie__ into a file named __session_cookie.txt__!

## License
This project is licensed under the MIT License - see the LICENSE file for details.