#[test]
fn part2_alg() {
    let input: Vec<super::Game> = vec![
        super::Game {
            id: 1,
            sets: vec![
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
            sets: vec![
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
                super::Set {
                    red_cubes: 4,
                    green_cubes: 4,
                    blue_cubes: 4,
                },
            ],
        },
    ];

    let expected = 91;
    let actual = super::part2_alg(input);
    assert_eq!(expected, actual);
}
