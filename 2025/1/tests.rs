// https://adventofcode.com/2025/day/1

use super::*;

static EXAMPLE_INPUT: &str = r#"
    L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82
"#;

#[test]
fn part1_example() {
    assert_eq!(get_part1_password(EXAMPLE_INPUT), 3);
}

#[test]
fn part2_example() {
    assert_eq!(get_part2_password(EXAMPLE_INPUT), 6);
}

#[test]
fn part2_edge_case1() {
    assert_eq!(get_part2_password("L51\nR2\nL1\nR1\nL1"), 4)
}

#[test]
fn part2_edge_case2() {
    assert_eq!(get_part2_password("L50\nL5\nR1"), 1)
}

#[test]
fn part2_edge_case3() {
    assert_eq!(get_part2_password("R1000"), 10)
}

#[test]
fn part2_edge_case4() {
    assert_eq!(get_part2_password("L1000"), 10)
}

#[test]
fn part2_edge_case5() {
    assert_eq!(get_part2_password("R50\nL100"), 2)
}

#[test]
fn part2_edge_case6() {
    assert_eq!(get_part2_password("L50\nR100"), 2)
}

#[test]
fn part2_edge_case7() {
    assert_eq!(get_part2_password("L50\nR101"), 2)
}
