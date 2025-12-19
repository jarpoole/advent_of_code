static EXAMPLE_INPUT: &str = r#"
    3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32
"#;

use super::*;

#[test]
fn merge_ranges() {
    // a overlap b from below
    assert_eq!(
        Range { lower: 1, upper: 3 }.merge(&Range { lower: 2, upper: 4 }),
        Some(Range { lower: 1, upper: 4 })
    );
    // b overlap a from below
    assert_eq!(
        Range { lower: 2, upper: 4 }.merge(&Range { lower: 1, upper: 3 }),
        Some(Range { lower: 1, upper: 4 })
    );
    // a fully contains b
    assert_eq!(
        Range { lower: 1, upper: 6 }.merge(&Range { lower: 2, upper: 4 }),
        Some(Range { lower: 1, upper: 6 })
    );
    // b fully contains a
    assert_eq!(
        Range { lower: 1, upper: 6 }.merge(&Range { lower: 2, upper: 4 }),
        Some(Range { lower: 1, upper: 6 })
    );
    // a equals b
    assert_eq!(
        Range { lower: 1, upper: 6 }.merge(&Range { lower: 1, upper: 6 }),
        Some(Range { lower: 1, upper: 6 })
    );
    // a overlaps b from above
    assert_eq!(
        Range { lower: 2, upper: 4 }.merge(&Range { lower: 1, upper: 3 }),
        Some(Range { lower: 1, upper: 4 })
    );
    // b overlaps a from above
    assert_eq!(
        Range { lower: 1, upper: 3 }.merge(&Range { lower: 2, upper: 4 }),
        Some(Range { lower: 1, upper: 4 })
    );
    // a is strictly smaller than b
    assert_eq!(
        Range { lower: 1, upper: 3 }.merge(&Range { lower: 4, upper: 6 }),
        None
    );
    // b is strictly smaller than a
    assert_eq!(
        Range { lower: 4, upper: 6 }.merge(&Range { lower: 1, upper: 3 }),
        None
    );
}

#[test]
fn part1_example() {
    let (ranges, ingredients) = parse_input(EXAMPLE_INPUT);
    assert_eq!(count_available_fresh_ingredients(&ranges, ingredients), 3)
}

#[test]
fn part2_example() {
    let (ranges, _) = parse_input(EXAMPLE_INPUT);
    assert_eq!(count_all_fresh_ingredients(ranges), 14)
}
