// https://adventofcode.com/2025/day/2

static EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

use super::*;

#[test]
fn part1_example() {
    assert_eq!(
        find_invalid_ids(EXAMPLE_INPUT, is_invalid_id_part1),
        1227775554
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        find_invalid_ids(EXAMPLE_INPUT, is_invalid_id_part2),
        4174379265
    );
}

#[test]
fn part2_invalid_ids() {
    // invalid ids
    assert!(is_invalid_id_part2("123123123"));
    assert!(is_invalid_id_part2("1111"));
    assert!(is_invalid_id_part2("68686868"));
    assert!(is_invalid_id_part2("22"));
    assert!(is_invalid_id_part2("333"));
    assert!(is_invalid_id_part2("453278453278"));
    assert!(is_invalid_id_part2("865865865865"));
    assert!(is_invalid_id_part2("700700"));
    assert!(is_invalid_id_part2("678906789067890"));

    // valid ids
    assert!(!is_invalid_id_part2("1"));
    assert!(!is_invalid_id_part2("234"));
}
