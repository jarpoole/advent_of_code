// https://adventofcode.com/2025/day/9

use std::cmp::{Ordering, max, min};

#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

enum ComparisonAxis {
    TopLeftBottomRight(i64, i64),
    TopRightBottomLeft,
}

#[derive(Copy, Clone, Debug)]
struct RedTile {
    x: i64,
    y: i64,
}

impl RedTile {
    fn parse(string: &str) -> Option<Self> {
        let mut coordinates = string.split(",");
        Some(RedTile {
            x: coordinates.next()?.parse::<i64>().ok()?,
            y: coordinates.next()?.parse::<i64>().ok()?,
        })
    }

    /*
    fn cmp(&self, other: &Self, axis: &ComparisonAxis) -> Ordering {
        let (self_x, self_y, other_x, other_y) = match axis {
            ComparisonAxis::TopRightBottomLeft => (self.x, self.y, other.x, other.y),
            ComparisonAxis::TopLeftBottomRight(max_x, max_y) => (
                max_x - self.x,
                max_y - self.y,
                max_x - other.x,
                max_y - other.y,
            ),
        };
        match (self_x + self_y).cmp(&(other_x + other_y)) {
            Ordering::Equal => max(self_x, self_y).cmp(&max(other_x, other_y)),
            ordering => ordering,
        }
    }
    */
}

fn max_rectangle_area(red_tiles: impl Iterator<Item = RedTile> + Clone) -> i64 {
    let max_x = red_tiles.clone().map(|tile| tile.x).max().unwrap();
    let max_y = red_tiles.clone().map(|tile| tile.y).max().unwrap();
    let mut vec: Vec<_> = red_tiles.collect();
    // find optimal top left corner
    vec.sort_by(
        |tile_a, tile_b| match (tile_a.x + tile_a.y).cmp(&(tile_b.x + tile_b.y)) {
            Ordering::Equal => max(tile_a.x, tile_a.y).cmp(&max(tile_b.x, tile_b.y)),
            ordering => ordering,
        },
    );
    let optimal_top_left = *vec.first().unwrap();
    dbg!(optimal_top_left);
    // find optimal top right corner
    vec.sort_by(|tile_a, tile_b| {
        match (max_x - tile_a.x + tile_a.y).cmp(&(max_x - tile_b.x + tile_b.y)) {
            Ordering::Equal => {
                max(max_x - tile_a.x, tile_a.y).cmp(&max(max_x - tile_b.x, tile_b.y))
            }
            ordering => ordering,
        }
    });
    let optimal_top_right = *vec.first().unwrap();
    dbg!(optimal_top_right);
    // find optimal bottom left
    vec.sort_by(|tile_a, tile_b| {
        match (tile_a.x + max_y - tile_a.y).cmp(&(tile_b.x + max_y - tile_b.y)) {
            Ordering::Equal => {
                max(tile_a.x, max_y - tile_a.y).cmp(&max(tile_b.x, max_y - tile_b.y))
            }
            ordering => ordering,
        }
    });
    let optimal_bottom_left = *vec.first().unwrap();
    dbg!(optimal_bottom_left);
    // find optimal bottom right
    vec.sort_by(|tile_a, tile_b| {
        match (max_x - tile_a.x + max_y - tile_a.y).cmp(&(max_x - tile_b.x + max_y - tile_b.y)) {
            Ordering::Equal => max(max_x - tile_a.x, max_y - tile_a.y)
                .cmp(&max(max_x - tile_b.x, max_y - tile_b.y)),
            ordering => ordering,
        }
    });
    let optimal_bottom_right = *vec.first().unwrap();
    dbg!(optimal_bottom_right);

    return max(
        ((optimal_top_right.x - optimal_bottom_left.x) + 1)
            * ((optimal_bottom_left.y - optimal_top_right.y) + 1),
        ((optimal_bottom_right.x - optimal_top_left.x) + 1)
            * ((optimal_bottom_right.y - optimal_top_left.y) + 1),
    );
}

fn get_red_tiles(input: &str) -> impl Iterator<Item = RedTile> + Clone {
    return input
        .split("\n")
        .filter_map(|line| (!line.is_empty()).then_some(line.trim()))
        .map(|line| RedTile::parse(line).expect(&format!("{line} should parse")));
}

fn main() {
    let input = helpers::get_input(2025, 9).unwrap();
    let maximum_area = max_rectangle_area(get_red_tiles(&input));

    println!("The largest rectangle possible is: {maximum_area}");
}
