#[test]
fn alg_part2() {
    let input: Vec<String> = vec![
        String::from("two1nine"),
        String::from("eightwothree"),
        String::from("abcone2threexyz"),
        String::from("xtwone3four"),
        String::from("4nineeightseven2"),
        String::from("zoneight234"),
        String::from("7pqrstsixteen"),
        String::from("abcfive"),
        String::from("oneabc"),
        String::from("3ksgl"),
    ];

    let expected = 380;
    let result = super::part2_alg(input);

    assert_eq!(result, expected);
}

#[test]
fn check_word_left() {
    let input: &str = "zsdftwo1nine";

    let result = super::parse_from_left(input, 0);
    assert!(result.is_none());

    let result = super::parse_from_left(input, 1);
    assert!(result.is_none());

    let result = super::parse_from_left(input, 2);
    assert!(result.is_none());

    let result = super::parse_from_left(input, 3);
    assert!(result.is_none());

    let result = super::parse_from_left(input, 4);
    assert!(result.is_some_and(|value| value == 2));
}

#[test]
fn check_word_right() {
    let input: &str = "zsdftwo1nine";

    let result = super::parse_from_right(input, input.len() - 1);
    assert!(result.is_some_and(|value| value == 9));

    let result = super::parse_from_right(input, input.len() - 2);
    assert!(result.is_none());

    let result = super::parse_from_right(input, input.len() - 3);
    assert!(result.is_none());

    let result = super::parse_from_right(input, input.len() - 4);
    assert!(result.is_none());

    let result = super::parse_from_right(input, input.len() - 5);
    assert!(result.is_some_and(|value| value == 1));
}
