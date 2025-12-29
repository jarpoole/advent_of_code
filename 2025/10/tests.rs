// https://adventofcode.com/2025/day/10

static EXAMPLE_INPUT: &str = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

use super::*;

#[test]
fn part1_example() {
    let machines = parse_machines(EXAMPLE_INPUT);
    assert_eq!(enable_all_machines(machines), 7);
}

#[test]
fn part2_example() {
    let machines = parse_machines(EXAMPLE_INPUT);
    assert_eq!(power_all_machines(machines), 33);
}
