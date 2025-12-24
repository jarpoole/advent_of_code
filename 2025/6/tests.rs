// https://adventofcode.com/2025/day/6

static EXAMPLE_INPUT: &str = r#"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"#;

use super::*;

#[test]
fn part1_example() {
    assert_eq!(part1(EXAMPLE_INPUT), 4277556)
}

#[test]
fn part2_example() {
    assert_eq!(part2(EXAMPLE_INPUT), 3263827)
}
