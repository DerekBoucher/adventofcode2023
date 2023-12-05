use clap::{Parser, ValueEnum};

mod test;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path towards the input file
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    alg_used: AlgorithmChoice,
}

#[derive(Debug, Clone, ValueEnum)]
enum AlgorithmChoice {
    Part1 = 1,
    Part2,
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    red_cubes: i32,
    green_cubes: i32,
    blue_cubes: i32,
}

const MAX_RED_CUBES: i32 = 12;
const MAX_GREEN_CUBES: i32 = 13;
const MAX_BLUE_CUBES: i32 = 14;

fn main() {
    env_logger::init();
    log::info!("Advent of code 2023!!!");
    log::info!("-----------------------------");
    log::info!("Day 2: Cube Conundrum");

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
    let games = parse_games(seperate_lines);
    log::debug!("Games: {:?}", games);

    _ = match args.alg_used {
        AlgorithmChoice::Part1 => part1_alg(games),
        AlgorithmChoice::Part2 => part2_alg(games),
    }
}

fn game_is_possible(sets: Vec<Set>) -> bool {
    for set in sets {
        if set.red_cubes > MAX_RED_CUBES
            || set.green_cubes > MAX_GREEN_CUBES
            || set.blue_cubes > MAX_BLUE_CUBES
        {
            return false;
        }
    }

    return true;
}

fn parse_games(lines: Vec<String>) -> Vec<Game> {
    let mut games: Vec<Game> = vec![];
    for line in lines {
        let line_segments: Vec<&str> = line.split(":").collect();

        let id: i32 = line_segments[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();

        let set_segments: Vec<&str> = line_segments[1]
            .trim()
            .split(";")
            .map(|set| set.trim())
            .collect();

        let mut current_game = Game {
            id: id,
            sets: vec![],
        };

        for set in set_segments {
            let mut current_set = Set {
                red_cubes: 0,
                green_cubes: 0,
                blue_cubes: 0,
            };

            let color_segments: Vec<&str> = set.split(",").map(|color| color.trim()).collect();

            for color in color_segments {
                let num_and_color: Vec<&str> = color.split(" ").collect();

                let num: i32 = num_and_color[0].parse::<i32>().unwrap();
                match num_and_color[1] {
                    "red" => current_set.red_cubes = num,
                    "green" => current_set.green_cubes = num,
                    "blue" => current_set.blue_cubes = num,
                    _ => panic!("Invalid color"),
                }
            }

            current_game.sets.push(current_set);
        }

        games.push(current_game);
    }

    return games;
}

fn part1_alg(games: Vec<Game>) -> i32 {
    let mut sum_ids = 0;

    for game in games {
        if !game_is_possible(game.sets) {
            log::debug!("Game with ID {} is impossible", game.id);
            continue;
        }

        sum_ids += game.id;
    }

    log::info!("Sum of IDs for possible games: {}", sum_ids);

    return sum_ids;
}

fn part2_alg(games: Vec<Game>) -> i32 {
    let mut sum_powers = 0;

    for game in games {
        let mut minimum_set: Set = Set {
            red_cubes: 0,
            green_cubes: 0,
            blue_cubes: 0,
        };

        for set in game.sets {
            if set.red_cubes > minimum_set.red_cubes {
                minimum_set.red_cubes = set.red_cubes;
            }

            if set.green_cubes > minimum_set.green_cubes {
                minimum_set.green_cubes = set.green_cubes;
            }

            if set.blue_cubes > minimum_set.blue_cubes {
                minimum_set.blue_cubes = set.blue_cubes;
            }
        }

        log::debug!("Minimum set: {:?}", minimum_set);

        sum_powers += minimum_set.red_cubes * minimum_set.green_cubes * minimum_set.blue_cubes;
    }

    log::info!("Sum of powers: {}", sum_powers);

    return sum_powers;
}
