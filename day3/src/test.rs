#[test]
fn part1_alg() {
    let grid: Vec<Vec<char>> = vec![
        vec!['.', '.', '.', '.'],
        vec!['1', '3', '%', '1'],
        vec!['.', '.', '.', '.'],
    ];

    assert_eq!(super::part1_alg(grid), 14);
}
