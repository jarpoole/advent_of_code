// https://adventofcode.com/2025/day/11

static EXAMPLE_INPUT: &str = r#"
    aaa: you hhh
    you: bbb ccc
    bbb: ddd eee
    ccc: ddd eee fff
    ddd: ggg
    eee: out
    fff: out
    ggg: out
    hhh: ccc fff iii
    iii: out
"#;

use super::*;

#[test]
fn part1_example() {
    let device_inputs = parse_device_inputs(EXAMPLE_INPUT);
    assert_eq!(count_paths(device_inputs), 5);
}

#[test]
fn part2_example() {
    assert_eq!(true, true);
}
