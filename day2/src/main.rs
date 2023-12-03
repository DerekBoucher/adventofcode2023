use clap::Parser;

mod test;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path towards the input file
    #[arg(short, long)]
    path: String,
}

#[derive(Debug)]
struct Game {
    id: i32,
    Sets: Vec<Set>,
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
            Sets: vec![],
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

            current_game.Sets.push(current_set);
        }

        games.push(current_game);
    }

    return games;
}
