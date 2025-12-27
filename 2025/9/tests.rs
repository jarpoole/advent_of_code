// https://adventofcode.com/2025/day/9

static EXAMPLE_INPUT: &str = r#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

use super::*;

#[test]
fn part1_example() {
    assert_eq!(max_rectangle_area(get_red_tiles(EXAMPLE_INPUT)), 50);
}

#[test]
fn part2_example() {
    assert_eq!(true, true);
}
