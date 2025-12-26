// https://adventofcode.com/2025/day/8

static EXAMPLE_INPUT: &str = r#"
    162,817,812
    57,618,57
    906,360,560
    592,479,940
    352,342,300
    466,668,158
    542,29,236
    431,825,988
    739,650,466
    52,470,668
    216,146,977
    819,987,18
    117,168,530
    805,96,715
    346,949,466
    970,615,88
    941,993,340
    862,61,35
    984,92,344
    425,690,689
"#;

use super::*;

#[test]
fn part1_example() {
    let junction_boxes = junction_boxes(EXAMPLE_INPUT);
    let closest_junction_boxes = closest_junction_boxes(junction_boxes);
    assert_eq!(
        get_multiplied_size_of_largest_circuits(closest_junction_boxes.iter().take(10)),
        40
    );
}

#[test]
fn part2_example() {
    let junction_boxes = junction_boxes(EXAMPLE_INPUT);
    let closest_junction_boxes = closest_junction_boxes(junction_boxes.clone());
    assert_eq!(
        get_multiplied_fully_connecting_x_coordinates(
            closest_junction_boxes.iter(),
            junction_boxes
        ),
        25272
    );
}
