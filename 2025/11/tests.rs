// https://adventofcode.com/2025/day/11

static EXAMPLE_INPUT_PART1: &str = r#"
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

static EXAMPLE_INPUT_PART2: &str = r#"
    svr: aaa bbb
    aaa: fft
    fft: ccc
    bbb: tty
    tty: ccc
    ccc: ddd eee
    ddd: hub
    hub: fff
    eee: dac
    dac: fff
    fff: ggg hhh
    ggg: out
    hhh: out
"#;

use super::*;

#[test]
fn part1_example() {
    let device_inputs = parse_device_inputs(EXAMPLE_INPUT_PART1);
    assert_eq!(count_part1_paths(device_inputs), 5);
}

#[test]
fn part2_example() {
    let device_inputs = parse_device_inputs(EXAMPLE_INPUT_PART2);
    assert_eq!(count_part2_paths(device_inputs), 2);
}
