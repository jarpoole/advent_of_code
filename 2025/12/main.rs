// https://adventofcode.com/2025/day/12

use std::collections::HashMap;

use ndarray::prelude::*;

#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Present {
    shape: Array2<bool>,
    number: usize,
}

impl Present {
    fn parse(s: &str) -> Self {
        let mut iter = s.split(':').map(str::trim);
        let number = iter
            .next()
            .and_then(|s| s.parse::<usize>().ok())
            .expect("each present should have a numeric label before the colon");
        let mut cols: usize = 0;
        let shape: Vec<_> = iter
            .next()
            .expect("each present should have a multi-line shape after the colon")
            .split_whitespace()
            .flat_map(|line| {
                cols = line.len();
                line.chars().map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Unexpected character '{c}' in the shape input"),
                })
            })
            .collect();
        let shape = Array2::from_shape_vec((shape.len() / cols, cols), shape)
            .expect("dimensions should always match number of bools in vec");
        Present { number, shape }
    }

    /// returns the unit area occupied by this present (number of #)
    fn area(&self) -> usize {
        self.shape // does not matter which rotation/mirroring of the present we use
            .iter()
            .map(|element| usize::from(*element))
            .sum()
    }

    /// returns the smallest bounding box which could contain this present
    fn size(&self) -> (usize, usize) {
        (self.shape.nrows(), self.shape.ncols())
    }
}

#[derive(Debug)]
struct Region<'a> {
    rows: usize,
    cols: usize,
    required_presents: HashMap<&'a Present, usize>,
}

impl<'a> Region<'a> {
    fn parse(s: &str, presents: &'a HashMap<usize, Present>) -> Self {
        let mut iter = s.split(&['x', ':', ' ']);
        let rows = iter
            .next()
            .and_then(|s| s.parse::<usize>().ok())
            .expect("each region should provide a number of rows before the 'x'");
        let cols = iter
            .next()
            .and_then(|s| s.parse::<usize>().ok())
            .expect("each region should provide a number of columns after the 'x'");
        Region {
            rows,
            cols,
            required_presents: iter
                .filter(|s| !s.is_empty())
                .enumerate()
                .map(|(i, s)| {
                    let present_count = s.parse::<usize>().unwrap_or_else(|_| {
                        panic!("Should parse {s} as a required number of present")
                    });
                    let present = presents
                        .get(&i)
                        .expect("all presents should be parsed at this step");
                    (present, present_count)
                })
                .collect(),
        }
    }

    // This problem is NP-complete so we need to take a shortcut for the full
    // puzzle input in order to conclude for certain that there are no solutions
    // for a given area. It turns out that the following two checks are sufficient
    // and that the full solver is only required for the example input which is far
    // smaller and can just be depth-first searched
    fn solve(&self) -> bool {
        // check if the presents could theoretically fit, ignoring packing
        let minimum_required_area: usize = self
            .required_presents
            .iter()
            .map(|(present, number)| present.area() * number)
            .sum();
        let available_area = self.rows * self.cols;
        if minimum_required_area > available_area {
            return false;
        }

        // check if the presents can always fit even without intersecting
        let minimum_present_bounds = self
            .required_presents
            .keys()
            .map(|key| key.size())
            .fold((0, 0), |acc, curr| {
                (usize::max(acc.0, curr.0), usize::max(acc.1, curr.1))
            });
        let num_possible_presents =
            (self.rows / minimum_present_bounds.0) * (self.cols / minimum_present_bounds.1);
        let num_required_presents: usize = self.required_presents.values().sum();
        if num_required_presents <= num_possible_presents {
            return true;
        }

        unimplemented!("full intersecting solver");
    }
}

fn parse_presents(input: &str) -> HashMap<usize, Present> {
    input
        .rsplit("\n\n")
        .skip(1)
        .map(|lines| {
            let present = Present::parse(lines);
            (present.number, present)
        })
        .collect()
}

fn parse_regions<'a>(input: &str, presents: &'a HashMap<usize, Present>) -> Vec<Region<'a>> {
    input
        .rsplit("\n\n")
        .take(1)
        .flat_map(|lines| {
            lines.split('\n').filter_map(|line| {
                let trimmed_line = line.trim();
                (!trimmed_line.is_empty()).then(|| Region::parse(trimmed_line, presents))
            })
        })
        .collect()
}

fn count_solvable_regions(input: &str) -> u32 {
    let presents = parse_presents(input);
    let regions = parse_regions(input, &presents);
    regions
        .iter()
        .map(|region| u32::from(region.solve()))
        .sum::<u32>()
}

fn main() {
    let input = helpers::get_input(2025, 12).unwrap();
    let solvable_regions = count_solvable_regions(&input);

    println!("The number of solvable regions is: {solvable_regions}")
}
