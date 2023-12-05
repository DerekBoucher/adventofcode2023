use clap::{Parser, ValueEnum};

mod test;

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
    let args = Args::parse();

    log::info!("Advent of code 2023!!!");
    log::info!("-----------------------------");
    log::info!("Day 3: Gear Ratios");
    log::info!("Loading input file: {}", args.path);

    let input = match std::fs::read_to_string(args.path) {
        Ok(input) => input,
        Err(e) => {
            log::error!("Error while reading input file: {}", e);
            return;
        }
    };

    let seperate_lines: Vec<String> = input.lines().map(|line| String::from(line)).collect();
    let symbol_grid = parse_into_grid(seperate_lines);

    for row in &symbol_grid {
        let mut row_str: String = "".to_string();
        let mut seperator: String = "".to_string();
        for c in row {
            row_str += String::from(format!("|{}", c)).as_str();
            seperator += "--";
        }
        log::debug!("{}", row_str);
        log::debug!("{}", seperator);
    }

    _ = match args.alg_used {
        AlgorithmChoice::Part1 => part1_alg(symbol_grid),
        _ => 0,
    }
}

fn parse_into_grid(lines: Vec<String>) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    return grid;
}

fn part1_alg(grid: Vec<Vec<char>>) -> i32 {
    let mut sum_parts = 0;

    for (y, row) in grid.iter().enumerate() {
        let mut current_digit_str = "".to_string();

        for (x, char) in row.iter().enumerate() {
            if !char.is_numeric() && current_digit_str.len() > 0 {
                sum_parts += search_around(
                    &grid,
                    x as i32 - current_digit_str.len() as i32,
                    y as i32,
                    current_digit_str.len() as i32,
                );

                current_digit_str = "".to_string();
                continue;
            }

            if char.is_numeric() {
                current_digit_str += format!("{}", char).as_str();
            }

            if x == row.len() - 1 {
                sum_parts += search_around(
                    &grid,
                    x as i32 - current_digit_str.len() as i32 + 1,
                    y as i32,
                    current_digit_str.len() as i32,
                );

                current_digit_str = "".to_string();
            }
        }
    }

    log::info!("Sum of all parts: {}", sum_parts);

    return sum_parts;
}

fn search_around(grid: &Vec<Vec<char>>, x: i32, y: i32, num_char_length: i32) -> i32 {
    let mut points_to_check: Vec<(i32, i32)> = vec![
        (x - 1, y),               // Right side
        (x + num_char_length, y), // Left side
    ];

    for y_loop in [y - 1, y + 1] {
        for x_loop in x - 1..x + num_char_length + 1 {
            points_to_check.push((x_loop, y_loop));
        }
    }

    for (x_loop, y_loop) in points_to_check {
        if x_loop < 0 || y_loop < 0 || x_loop >= grid[0].len() as i32 || y_loop >= grid.len() as i32
        {
            continue;
        }

        // If the point is a dot or a number, skip it
        if grid[y_loop as usize][x_loop as usize] == '.'
            || grid[y_loop as usize][x_loop as usize].is_numeric()
        {
            continue;
        }

        // Else we have a hit
        let mut num_str = "".to_string();
        for x_find in x..x + num_char_length {
            num_str += format!("{}", grid[y as usize][x_find as usize]).as_str();
        }

        return num_str.parse::<i32>().unwrap();
    }

    return 0;
}
