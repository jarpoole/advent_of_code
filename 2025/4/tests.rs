static EXAMPLE_INPUT: &str = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.\n";

use super::*;

#[test]
fn part1_example() {
    let rolls = parse_input(EXAMPLE_INPUT);
    let (_, num_directly_accessible_rolls) = get_num_directly_accessible_rolls(&rolls);
    assert_eq!(num_directly_accessible_rolls, 13);
}

#[test]
fn part2_example() {
    let rolls = parse_input(EXAMPLE_INPUT);
    assert_eq!(get_num_accessible_rolls(rolls), 43)
}
