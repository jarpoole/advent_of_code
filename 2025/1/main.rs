// https://adventofcode.com/2025/day/1

#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

#[derive(Debug)]
struct Rotation(i32);

impl Rotation {
    fn parse(input: &str) -> Option<Self> {
        let direction_char = input.chars().nth(0)?;
        let rotations = &input[1..].parse::<i32>().ok()?;
        let direction = match direction_char {
            'R' => 1,
            'L' => -1,
            _ => return None,
        };
        Some(Rotation(direction * rotations))
    }
}

// use signed integers even though none of these values can be negative because
// it ensures the arithmetic can be performed without casting
struct Dial {
    /// the number of ticks on the dial (with values from 0 to size-1)
    size: i32,
    /// the number of times the dial lands on exactly 0
    exactly_zero_count: i32,
    /// the number of times the dial passes through 0 without stopping
    through_zero_count: i32,
    /// the current position of the dial
    position: i32,
}

impl Dial {
    fn new(size: i32) -> Self {
        Dial {
            size,
            exactly_zero_count: 0,
            through_zero_count: 0,
            position: 50,
        }
    }

    fn rotate(&mut self, rotation: &Rotation) {
        // compute next position
        let raw_next_position = rotation.0 + self.position;
        let next_position = raw_next_position.rem_euclid(self.size);

        // update tracking data
        if next_position == 0 {
            self.exactly_zero_count += 1;
        }
        self.through_zero_count += raw_next_position.div_euclid(self.size).abs();
        if (raw_next_position < 0 && self.position == 0)
            || (raw_next_position >= 100 && next_position == 0)
        {
            self.through_zero_count -= 1;
        }

        // update position
        self.position = next_position;
    }
}

fn simulate_dial_position(input: &str) -> Dial {
    let mut state = Dial::new(100);
    input
        .split_whitespace()
        .filter_map(|line| {
            (!line.is_empty()).then_some(
                Rotation::parse(line.trim()).unwrap_or_else(|| panic!("{line} should parse")),
            )
        })
        .for_each(|rotation| state.rotate(&rotation));
    state
}

fn get_part1_password(input: &str) -> i32 {
    simulate_dial_position(input).exactly_zero_count
}
fn get_part2_password(input: &str) -> i32 {
    let result = simulate_dial_position(input);
    result.exactly_zero_count + result.through_zero_count
}

fn main() {
    let input = helpers::get_input(2025, 1).unwrap();
    println!("The part 1 password is: {}", get_part1_password(&input));
    println!("The part 2 password is: {}", get_part2_password(&input));
}
