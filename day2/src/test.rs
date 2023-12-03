#[test]
fn parse_games() {
    let input: Vec<String> = vec![
        String::from(
            "Game 1: 1 red, 1 green, 1 blue; 2 red, 2 green, 2 blue; 3 red, 3 green, 3 blue",
        ),
        String::from(
            "Game 2: 1 red, 1 green, 1 blue; 2 red, 2 green, 2 blue; 3 red, 3 green, 3 blue",
        ),
    ];

    let expected: Vec<super::Game> = vec![
        super::Game {
            id: 1,
            Sets: vec![
                super::Set {
                    red_cubes: 1,
                    green_cubes: 1,
                    blue_cubes: 1,
                },
                super::Set {
                    red_cubes: 2,
                    green_cubes: 2,
                    blue_cubes: 2,
                },
                super::Set {
                    red_cubes: 3,
                    green_cubes: 3,
                    blue_cubes: 3,
                },
            ],
        },
        super::Game {
            id: 2,
            Sets: vec![
                super::Set {
                    red_cubes: 1,
                    green_cubes: 1,
                    blue_cubes: 1,
                },
                super::Set {
                    red_cubes: 2,
                    green_cubes: 2,
                    blue_cubes: 2,
                },
                super::Set {
                    red_cubes: 3,
                    green_cubes: 3,
                    blue_cubes: 3,
                },
            ],
        },
    ];

    let actual = super::parse_games(input);
}
