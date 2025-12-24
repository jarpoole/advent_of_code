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
fn part2_() {
    assert_eq!(get_part2_password("L51\nR2\nL1\nR1\nL1"), 4)
}
