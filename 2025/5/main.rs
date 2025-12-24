// https://adventofcode.com/2025/day/5

use std::{cmp, str};

#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

// when `PartialOrd` is derived on structs it compares fields in lexicographical
// order which is desirable here to sort by lower bound before upper bound
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Range {
    lower: u64,
    upper: u64,
}

impl Range {
    /// construct a [`Range`] from an input string in the format <lower>-<upper>  
    fn new(range: &str) -> Self {
        let mut bounds = range.split("-").map(|num| {
            num.parse::<u64>()
                .expect("all characters before or after the hyphen should form a valid integer")
        });
        Range {
            lower: bounds
                .next()
                .expect("iterator should yield exactly two values"),
            upper: bounds
                .next()
                .expect("iterator should yield exactly two values"),
        }
    }

    /// checks if an ingredient id is inside the [`Range`]
    fn contains(&self, ingredient: u64) -> bool {
        ingredient >= self.lower && ingredient <= self.upper
    }

    /// merges two [`Range`]s together if they overlap
    fn merge(&self, other: &Range) -> Option<Range> {
        (self.upper >= other.lower && self.lower <= other.upper).then_some(Range {
            lower: cmp::min(self.lower, other.lower),
            upper: cmp::max(self.upper, other.upper),
        })
    }

    /// computes the number of ingredient ids in the [`Range`]
    fn size(&self) -> u64 {
        return self.upper - self.lower + 1;
    }
}

fn count_available_fresh_ingredients<'a>(
    ranges: &Vec<Range>,
    ingredients: impl Iterator<Item = &'a str>,
) -> u64 {
    return ingredients.fold(0, |mut accumulator, ingredient_str| {
        let ingredient = ingredient_str
            .parse::<u64>()
            .expect("ingredient ids should always be valid integers");
        if ranges.iter().any(|range| range.contains(ingredient)) {
            accumulator += 1
        }
        accumulator
    });
}

fn count_all_fresh_ingredients(mut ranges: Vec<Range>) -> u64 {
    ranges.sort();
    let merged_ranges = ranges.iter().fold(Vec::new(), |mut accumulator, range| {
        if let Some(current_largest_range) = accumulator.last_mut()
            && let Some(merged_range) = range.merge(current_largest_range)
        {
            // because we sorted the ranges, if the current range overlaps with
            // the largest range in the accumulator we can just merge them
            // together
            *current_largest_range = merged_range;
        } else {
            // if this is the first range under consideration or it is non-overlapping
            // (and therefore strictly larger than the previous one) then keep it
            accumulator.push(*range);
        }
        return accumulator;
    });
    merged_ranges.iter().map(|range| range.size()).sum()
}

fn parse_input(input: &str) -> (Vec<Range>, impl Iterator<Item = &str>) {
    let mut inputs = input.split("\n\n");
    let ranges = inputs
        .next()
        .expect("ranges should always be located before the empty newline")
        .split("\n")
        .filter_map(|ingredient_str| {
            let trimmed_str = ingredient_str.trim();
            (!trimmed_str.is_empty()).then_some(trimmed_str)
        })
        .map(|range| Range::new(range))
        .collect::<Vec<_>>();

    let ingredients = inputs
        .next()
        .expect("available ingredients should always be located after the empty newline")
        .split("\n")
        .filter_map(|ingredient_str| {
            let trimmed_str = ingredient_str.trim();
            (!trimmed_str.is_empty()).then_some(trimmed_str)
        });

    return (ranges, ingredients);
}

fn main() {
    let input = helpers::get_input(2025, 5).unwrap();
    let (ranges, ingredients) = parse_input(&input);

    println!(
        "Available fresh ingredients: {}",
        count_available_fresh_ingredients(&ranges, ingredients)
    );

    println!(
        "All fresh ingredients: {}",
        count_all_fresh_ingredients(ranges)
    );
}
