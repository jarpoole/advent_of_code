// https://adventofcode.com/2025/day/12

use std::collections::HashMap;

use ndarray::prelude::*;

#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

fn rotate_matrix_90_degrees_clockwise<T: Copy>(matrix: &Array2<T>) -> Array2<T> {
    let mut rotated = (*matrix).clone();
    rotated.swap_axes(0, 1);
    for mut row in rotated.axis_iter_mut(Axis(0)) {
        row.invert_axis(Axis(0));
    }
    rotated
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Present {
    shapes: [Array2<bool>; 4],
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
            .split("\n")
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
        let rotated_shape1 = rotate_matrix_90_degrees_clockwise(&original_shape);
        let rotated_shape2 = rotate_matrix_90_degrees_clockwise(&rotated_shape1);
        let rotated_shape3 = rotate_matrix_90_degrees_clockwise(&rotated_shape2);
        Present {
            number,
            shapes: [
                original_shape,
                rotated_shape1,
                rotated_shape2,
                rotated_shape3,
            ],
        }
    }
}

#[derive(Debug)]
struct Region<'a> {
    space: Array2<bool>,
    required_presents: HashMap<&'a Present, usize>,
}

impl<'a> Region<'a> {
    fn parse(s: &str, presents: &'a HashMap<usize, Present>) -> Self {
        let mut iter = s.split(&['x', ':', ' ']);
        let rows = iter.next().and_then(|s| s.parse::<usize>().ok()).expect("");
        let cols = iter.next().and_then(|s| s.parse::<usize>().ok()).expect("");
        Region {
            space: Array2::from_elem((rows, cols), false),
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

    fn solve(&self) {
        // self.space.windows()
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

fn main() {
    let input = helpers::get_input(2025, 12).unwrap();

    let presents = parse_presents(&input);
    let regions = parse_regions(&input, &presents);

    //println!("presents: {:?}", presents);
    //println!("regions: {:?}", regions);
    let x = Array2::from_elem((2, 3), 1);
}
