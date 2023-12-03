extern crate log;
extern crate phf;

use clap::{Parser, ValueEnum};

mod test;

static WORD_TO_DIGIT: phf::Map<&'static str, i32> = phf::phf_map! {
    "zero" => 0,
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

#[derive(Debug, Clone, ValueEnum)]
enum AlgorithmChoice {
    Part1 = 1,
    Part2,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path towards the input file
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    alg_used: AlgorithmChoice,
}

fn main() {
    env_logger::init();
    log::info!("Advent of code 2023!!!");
    log::info!("-----------------------------");
    log::info!("Day 1: Report Repair");

    let args = Args::parse();
    log::info!("Loading input file: {}", args.path);

    let input = match std::fs::read_to_string(args.path) {
        Ok(input) => input,
        Err(e) => {
            log::error!("Error while reading input file: {}", e);
            return;
        }
    };

    let seperate_lines: Vec<String> = input.lines().map(|line| String::from(line)).collect();

    let mut _calibration_sum: i32 = 0;
    match args.alg_used {
        AlgorithmChoice::Part1 => _calibration_sum = part1_alg(seperate_lines),
        AlgorithmChoice::Part2 => _calibration_sum = part2_alg(seperate_lines),
    }

    log::info!("Calibration sum: {}", _calibration_sum);
}

fn part1_alg(lines: Vec<String>) -> i32 {
    let mut calibration_sum: i32 = 0;

    for line in lines {
        let chars: Vec<char> = line.chars().map(|char| char).collect();

        // Use a two pointer solution -> one at the begining of the string and one at the end to find the enclosing integers
        let mut left = 0;
        let mut right = chars.len() - 1;

        while left <= right {
            let left_char = chars[left];
            let right_char = chars[right];

            if left_char.is_numeric() && right_char.is_numeric() {
                let left_int = left_char.to_digit(10).unwrap() as i32;
                let right_int = right_char.to_digit(10).unwrap() as i32;

                log::debug!("left: {}", left_int);
                log::debug!("right: {}", right_int);

                let mut calibration_value: i32 = left_int * 10;
                calibration_value += right_int;
                log::debug!("calibration value: {}", calibration_value);

                calibration_sum += calibration_value;
                break;
            }

            // Handle cases where there's only one integer
            // in the string.
            if left == right {
                let mut unique: i32 = 0;
                if left_char.is_numeric() {
                    unique = left_char.to_digit(10).unwrap() as i32;
                }

                if right_char.is_numeric() {
                    unique = right_char.to_digit(10).unwrap() as i32;
                }

                let mut calibration_value: i32 = unique * 10;
                calibration_value += unique;

                log::debug!("Unique integer value: {}", unique);
                log::debug!("Unique integer calibration unit: {}", calibration_value);
                calibration_sum += calibration_value;
            }

            if !left_char.is_numeric() {
                left += 1;
            }

            if !right_char.is_numeric() {
                right -= 1;
            }
        }
    }

    return calibration_sum;
}

fn part2_alg(lines: Vec<String>) -> i32 {
    let mut calibration_sum = 0;

    for line in lines {
        let chars: Vec<char> = line.chars().map(|char| char).collect();

        let mut left_ptr = 0;
        let mut right_ptr = chars.len() - 1;

        let mut left_value = 0;
        let mut right_value = 0;

        while left_ptr <= right_ptr {
            if left_value == 0 {
                match parse_from_left(line.as_str(), left_ptr) {
                    Some(value) => {
                        left_value = value;
                    }
                    None => {
                        left_ptr += 1;
                    }
                }
            }

            if right_value == 0 {
                match parse_from_right(line.as_str(), right_ptr) {
                    Some(value) => {
                        right_value = value;
                    }
                    None => {
                        right_ptr -= 1;
                    }
                }
            }

            if left_value > 0 && right_value > 0 {
                let mut calibration_value: i32 = left_value * 10;
                calibration_value += right_value;

                log::debug!("left: {}", left_value);
                log::debug!("right: {}", right_value);
                log::debug!("calibration value: {}", calibration_value);

                calibration_sum += calibration_value;
                break;
            }
        }
    }

    return calibration_sum;
}

fn parse_from_left(line: &str, index: usize) -> Option<i32> {
    // First check that the character at the index is numeric
    if line.chars().nth(index).unwrap().is_numeric() {
        return Some(line.chars().nth(index).unwrap().to_digit(10).unwrap() as i32);
    }

    // Then parse the word from the left
    for (word, value) in WORD_TO_DIGIT.entries() {
        if index + word.len() > line.len() {
            continue;
        }

        let sub_string = &line[index..(index + word.len())];
        if *sub_string == **word {
            return Some(*value);
        }
    }

    return None;
}

fn parse_from_right(line: &str, index: usize) -> Option<i32> {
    // First check that the character at the index is numeric
    if line.chars().nth(index).unwrap().is_numeric() {
        return Some(line.chars().nth(index).unwrap().to_digit(10).unwrap() as i32);
    }

    // Then parse the word from the left
    for (word, value) in WORD_TO_DIGIT.entries() {
        if index + 1 < word.len() {
            continue;
        }

        let sub_string = &line[(index + 1) - word.len()..(index + 1)];
        if *sub_string == **word {
            return Some(*value);
        }
    }

    return None;
}
