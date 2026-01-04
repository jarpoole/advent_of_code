// https://adventofcode.com/2025/day/9

use itertools::Itertools;

#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, PartialEq)]
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
}

type Rectangle = (RedTile, RedTile, i64);

struct HorizontalLine {
    x1: i64,
    x2: i64,
    y: i64,
}
struct VerticalLine {
    x: i64,
    y1: i64,
    y2: i64,
}
enum Line {
    Horizontal(HorizontalLine),
    Vertical(VerticalLine),
}

impl Line {
    fn from(tile1: &RedTile, tile2: &RedTile) -> Option<Self> {
        Line::new(tile1.x, tile1.y, tile2.x, tile2.y)
    }
    fn new(x1: i64, y1: i64, x2: i64, y2: i64) -> Option<Self> {
        if x1 == x2 {
            return Some(Line::Vertical(VerticalLine {
                x: x1, // choice is arbitrary, could by x2
                y1: i64::min(y1, y2),
                y2: i64::max(y1, y2),
            }));
        } else if y1 == y2 {
            return Some(Line::Horizontal(HorizontalLine {
                x1: i64::min(x1, x2),
                x2: i64::max(x1, x2),
                y: y1, // choice is arbitrary, could by y2
            }));
        } else {
            return None;
        }
    }
    fn intersects(&self, other: &Self) -> bool {
        match self {
            Line::Horizontal(HorizontalLine { x1, x2, y }) => match other {
                Line::Horizontal(_) => false,
                Line::Vertical(VerticalLine { x, y1, y2 }) => x1 < x && x < x2 && y1 < y && y < y2,
            },
            Line::Vertical(VerticalLine { x, y1, y2 }) => match other {
                Line::Vertical(_) => false,
                Line::Horizontal(HorizontalLine { x1, x2, y }) => {
                    x1 < x && x < x2 && y1 < y && y < y2
                }
            },
        }
    }
}

fn get_red_tiles(input: &str) -> impl Iterator<Item = RedTile> + Clone {
    return input
        .split("\n")
        .filter_map(|line| (!line.is_empty()).then_some(line.trim()))
        .map(|line| RedTile::parse(line).expect(&format!("{line} should parse")));
}

fn rectangle_area(corner1: &RedTile, corner2: &RedTile) -> i64 {
    return ((corner1.x - corner2.x).abs() + 1) * ((corner1.y - corner2.y).abs() + 1);
}

fn max_rectangles<'a, T: IntoIterator<Item = &'a RedTile>>(red_tiles: T) -> Vec<Rectangle>
where
    <T as IntoIterator>::IntoIter: Clone,
{
    red_tiles
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| (*a, *b, rectangle_area(&a, &b)))
        .sorted_by(|(_, _, a_area), (_, _, b_area)| a_area.cmp(&b_area))
        .rev()
        .collect()
}

fn max_rectangle_area<'a>(rectangles: impl IntoIterator<Item = &'a Rectangle>) -> i64 {
    rectangles
        .into_iter()
        .next()
        .expect("more than two red tiles will always be provided so there will always be a maximum area")
        .2
}

fn max_red_or_green_rectangle_area<
    'a,
    'b,
    RECT: IntoIterator<Item = &'a Rectangle>,
    RED: IntoIterator<Item = &'b RedTile> + Copy,
>(
    rectangles: RECT,
    red_tiles: RED,
) -> i64
where
    <RED as IntoIterator>::IntoIter: Clone + ExactSizeIterator,
{
    let max_rectangle = rectangles.into_iter().find(|(corner1, corner2, _)| {
        let side1 = Line::new(corner1.x, corner1.y, corner1.x, corner2.y).expect("");
        let side2 = Line::new(corner1.x, corner2.y, corner2.x, corner2.y).expect("");
        let side3 = Line::new(corner2.x, corner2.y, corner2.x, corner1.y).expect("");
        let side4 = Line::new(corner2.x, corner1.y, corner1.x, corner1.y).expect("");
        red_tiles
            .into_iter()
            .circular_tuple_windows()
            .find(|(a, b)| {
                let line = Line::from(a, b).expect("");
                return line.intersects(&side1)
                    || line.intersects(&side2)
                    || line.intersects(&side3)
                    || line.intersects(&side4);
            })
            .is_none()
    });
    return max_rectangle
        .expect("should always find a rectangle that fits in the green area as this is what the problem asks for")
        .2;
}

fn main() {
    let input = helpers::get_input(2025, 9).unwrap();
    let red_tiles: Vec<RedTile> = get_red_tiles(&input).collect();
    let max_rectangles = max_rectangles(&red_tiles);
    println!(
        "The largest rectangle possible is: {}",
        max_rectangle_area(&max_rectangles)
    );
    println!(
        "The largest rectangle including only red and green tiles is: {}",
        max_red_or_green_rectangle_area(&max_rectangles, &red_tiles)
    );
}
