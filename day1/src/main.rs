extern crate log;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path towards the input file
    #[arg(short, long)]
    path: String,

    /// Determines the algorithm to use based on the problem part.
    #[arg(short, long, default_value_t = 1)]
    alg_used: u32,
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

    if args.alg_used == 1 {
        part1_alg(seperate_lines);
    }
}

fn part1_alg(lines: Vec<String>) {
    let mut calibration_sum = 0;

    for line in lines {
        let chars: Vec<char> = line.chars().map(|char| char).collect();

        // Use a two pointer solution -> one at the begining of the string and one at the end to find the enclosing integers
        let mut left = 0;
        let mut right = chars.len() - 1;

        while left <= right {
            let left_char = chars[left];
            let right_char = chars[right];

            if left_char.is_numeric() && right_char.is_numeric() {
                let left_int = left_char.to_digit(10).unwrap();
                let right_int = right_char.to_digit(10).unwrap();

                log::debug!("left: {}", left_int);
                log::debug!("right: {}", right_int);

                let mut calibration_value = left_int * 10;
                calibration_value += right_int;
                log::debug!("calibration value: {}", calibration_value);

                calibration_sum += calibration_value;
                break;
            }

            // Handle cases where there's only one integer
            // in the string.
            if left == right {
                let mut unique = 0;
                if left_char.is_numeric() {
                    unique = left_char.to_digit(10).unwrap();
                }

                if right_char.is_numeric() {
                    unique = right_char.to_digit(10).unwrap();
                }

                let mut calibration_value = unique * 10;
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

    log::info!("Calibration sum: {}", calibration_sum);
}
