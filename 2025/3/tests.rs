static EXAMPLE_INPUT: &str = r#"
    987654321111111
    811111111111119
    234234234234278
    818181911112111
"#;

use super::*;

#[test]
fn part1_example() {
    assert_eq!(get_max_joltage(EXAMPLE_INPUT, 2), 357);
}

#[test]
fn part2_example() {
    assert_eq!(get_max_joltage(EXAMPLE_INPUT, 12), 3121910778619);
}
