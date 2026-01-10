// https://adventofcode.com/2025/day/12

use std::collections::HashMap;

use itertools::Itertools;
use ndarray::prelude::*;

#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

static SHAPE_PERMUTATIONS: usize = 8;

fn flip_matrix<T: Copy>(matrix: &Array2<T>) -> Array2<T> {
    let mut flipped = matrix.clone();
    for mut row in flipped.axis_iter_mut(Axis(0)) {
        row.invert_axis(Axis(0));
    }
    flipped
}

fn rotate_matrix_90_degrees_clockwise<T: Copy>(matrix: &Array2<T>) -> Array2<T> {
    let mut rotated = matrix.clone();
    rotated.swap_axes(0, 1);
    for mut row in rotated.axis_iter_mut(Axis(0)) {
        row.invert_axis(Axis(0));
    }
    rotated
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Present {
    shapes: [Array2<bool>; SHAPE_PERMUTATIONS],
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
        let original_shape = Array2::from_shape_vec((shape.len() / cols, cols), shape)
            .expect("dimensions should always match number of bools in vec");

        let original_flipped = flip_matrix(&original_shape);
        let rotated_shape1 = rotate_matrix_90_degrees_clockwise(&original_shape);
        let rotated_flipped_shape1 = flip_matrix(&rotated_shape1);
        let rotated_shape2 = rotate_matrix_90_degrees_clockwise(&rotated_shape1);
        let rotated_flipped_shape2 = flip_matrix(&rotated_shape2);
        let rotated_shape3 = rotate_matrix_90_degrees_clockwise(&rotated_shape2);
        let rotated_flipped_shape3 = flip_matrix(&rotated_shape3);
        Present {
            number,
            shapes: [
                original_shape,
                original_flipped,
                rotated_shape1,
                rotated_flipped_shape1,
                rotated_shape2,
                rotated_flipped_shape2,
                rotated_shape3,
                rotated_flipped_shape3,
            ],
        }
    }

    /// returns the unit area occupied by this present (number of #)
    fn area(&self) -> usize {
        self.shapes[0] // does not matter which rotation/mirroring of the present we use
            .iter()
            .map(|element| usize::from(*element))
            .sum()
    }

    /// returns the smallest bounding box which could contain this present
    fn size(&self) -> (usize, usize) {
        (
            self.shapes[0].len_of(Axis(0)),
            self.shapes[0].len_of(Axis(1)),
        )
    }
}

#[derive(Clone)]
struct PresentPlacer<'a> {
    present: &'a Present,
    row: usize,
    col: usize,
    shape: usize,
    max_row: usize,
    max_col: usize,
}

impl<'a> PresentPlacer<'a> {
    fn new(present: &'a Present, max_row: usize, max_col: usize) -> Self {
        PresentPlacer {
            present,
            row: 0,
            col: 0,
            shape: 0,
            max_row,
            max_col,
        }
    }
}

impl Iterator for PresentPlacer<'_> {
    type Item = (Array2<bool>, usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.row + 3 > self.max_row {
            return None;
        }
        let next_placement = (self.present.shapes[self.shape].clone(), self.row, self.col);
        self.shape = (self.shape + 1) % SHAPE_PERMUTATIONS;
        if self.shape == 0 {
            self.col = (self.col + 1) % (self.max_col - 3 + 1);
            if self.col == 0 {
                self.row += 1;
            }
        }
        Some(next_placement)
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
        let rows = iter.next().and_then(|s| s.parse::<usize>().ok()).expect("");
        let cols = iter.next().and_then(|s| s.parse::<usize>().ok()).expect("");
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
                    let present = presents.get(&i).expect("");
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

        // attempt to find a packed solution
        let mut region_space = Array2::from_elem((self.rows, self.cols), false);
        self.required_presents
            .iter()
            .flat_map(|(present, &present_count)| {
                (0..present_count).map(|_| PresentPlacer::new(present, self.rows, self.cols))
            })
            .permutations(num_required_presents)
            .any(|present_placers| {
                present_placers
                    .into_iter()
                    .multi_cartesian_product()
                    .any(|shapes| {
                        //dbg!(&shapes);
                        for (shape, row_offset, col_offset) in shapes {
                            let mut slice = region_space.slice_mut(s![
                                row_offset..row_offset + shape.nrows(),
                                col_offset..col_offset + shape.ncols()
                            ]);
                            if slice.iter().map(|element| u32::from(*element)).sum::<u32>() != 0 {
                                //println!("shapes overlap");
                                return false; // shapes overlap
                            };
                            slice.assign(&shape);
                        }
                        // reset the temp buffer back to its starting state
                        region_space.fill(false);
                        true
                    })
            })
        // unimplemented!("full intersecting solver");
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
