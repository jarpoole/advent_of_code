// https://adventofcode.com/2025/day/9

use geo::{Contains, Coord, LineString, Polygon, Rect};
use itertools::Itertools;
use rayon::prelude::*;

#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, PartialEq)]
struct RedTile {
    x: i32,
    y: i32,
}

impl RedTile {
    fn parse(string: &str) -> Option<Self> {
        let mut coordinates = string.split(",");
        Some(RedTile {
            x: coordinates.next()?.parse::<i32>().ok()?,
            y: coordinates.next()?.parse::<i32>().ok()?,
        })
    }
}

type Rectangle = (RedTile, RedTile, i64);

fn get_red_tiles(input: &str) -> Vec<RedTile> {
    input
        .split("\n")
        .filter_map(|line| (!line.is_empty()).then_some(line.trim()))
        .map(|line| RedTile::parse(line).unwrap_or_else(|| panic!("{line} should parse")))
        .collect()
}

fn rectangle_area(corner1: &RedTile, corner2: &RedTile) -> i64 {
    // add one to each side length because a minimum sized rectangle where
    // both corners are the same point should still be considered to have
    // an area of 1 instead of 0
    i64::from((corner1.x - corner2.x).abs() + 1) * i64::from((corner1.y - corner2.y).abs() + 1)
}

fn max_rectangles<'a>(
    red_tiles: impl Iterator<Item = &'a RedTile> + Clone,
) -> impl Iterator<Item = Rectangle> {
    red_tiles
        .tuple_combinations()
        .map(|(a, b)| (*a, *b, rectangle_area(a, b)))
        .sorted_by(|(_, _, a_area), (_, _, b_area)| a_area.cmp(b_area))
        .rev()
}

fn max_rectangle_area<'a, T: IntoIterator<Item = &'a RedTile>>(red_tiles: T) -> i64
where
    <T as IntoIterator>::IntoIter: Clone,
{
    max_rectangles(red_tiles.into_iter())
        .next()
        .expect("more than two red tiles will always be provided so there will always be a maximum area")
        .2
}

fn max_red_or_green_rectangle_area<'a, T: IntoIterator<Item = &'a RedTile> + Clone>(
    red_tiles: T,
) -> i64
where
    <T as IntoIterator>::IntoIter: Clone,
{
    let polygon = Polygon::new(
        LineString::from_iter(
            red_tiles
                .clone()
                .into_iter()
                // The DE-9IM methods from "geo" are unfortunately only defined for floats
                .map(|tile| (f64::from(tile.x), f64::from(tile.y))),
        ),
        vec![], // no inside holes
    );

    // collect here because there does not seem to be a good way to
    // convert an opaque iterator type to a parallel iterator
    let max_rectangles: Vec<Rectangle> = max_rectangles(red_tiles.into_iter()).collect();
    let max_rectangle = max_rectangles
        .par_iter()
        .find_first(|(corner1, corner2, _)| {
            let rectangle = Rect::new(
                Coord {
                    x: f64::from(corner1.x),
                    y: f64::from(corner1.y),
                },
                Coord {
                    x: f64::from(corner2.x),
                    y: f64::from(corner2.y),
                },
            );
            polygon.contains(&rectangle)
        });
    max_rectangle
        .expect("should always find a rectangle that fits in the green area as this is what the problem asks for")
        .2
}

fn main() {
    let input = helpers::get_input(2025, 9).unwrap();
    let red_tiles = get_red_tiles(&input);
    println!(
        "The largest rectangle possible is: {}",
        max_rectangle_area(&red_tiles)
    );
    println!(
        "The largest rectangle including only red and green tiles is: {}",
        max_red_or_green_rectangle_area(&red_tiles)
    );
}
