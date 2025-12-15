#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

struct State {
    exactly_zero_count: u32, // part 1
    zero_count: u32,         // part 2
    position: u8,
}

impl State {
    fn new() -> State {
        State {
            exactly_zero_count: 0,
            zero_count: 0,
            position: 50,
        }
    }
}

fn simulate_dial_position(input: &str) -> State {
    return input.split("\n").map(|line| line.trim()).fold(
        State::new(),
        |mut accumulator, current| {
            if let Some(direction_char) = current.chars().nth(0)
                && let Ok(rotations) = &current[1..].parse::<i32>()
            {
                let direction = match direction_char {
                    'R' => 1,
                    'L' => -1,
                    _ => 0,
                };
                let next_position = (direction * rotations) + accumulator.position as i32;
                accumulator.zero_count += next_position.div_euclid(100).abs() as u32;
                if next_position < 0 && accumulator.position == 0 {
                    accumulator.zero_count -= 1;
                }
                accumulator.position = next_position.rem_euclid(100) as u8;
                if accumulator.position == 0 {
                    accumulator.exactly_zero_count += 1;
                }
                println!(
                    "pos={}, count={}",
                    accumulator.position, accumulator.zero_count
                );
            }
            return accumulator;
        },
    );
}

fn get_part1_password(input: &str) -> u32 {
    return simulate_dial_position(input).exactly_zero_count;
}
fn get_part2_password(input: &str) -> u32 {
    return simulate_dial_position(input).zero_count;
}

fn main() {
    let input = helpers::get_input(2025, 1).unwrap();
    println!("The part 1 password is: {}", get_part1_password(&input));
    println!("The part 2 password is: {}", get_part2_password(&input));
}
