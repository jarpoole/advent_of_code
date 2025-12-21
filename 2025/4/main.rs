#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

use ndarray::{Array2, ArrayBase, Zip, array};
use ndarray_conv::*;

fn parse_input(input: &str) -> Array2<u8> {
    let mut rows = 1;
    let array = ArrayBase::from_iter(input.trim().chars().filter_map(|char| match char {
        '.' => Some(0),
        '@' => Some(1),
        '\n' => {
            rows += 1;
            None
        }
        c => panic!("should never encounter char '{}' in the input", c),
    }));
    // subtract the number of rows as each row adds one extra character for the newline
    let columns = (input.len() - rows) / rows;
    return array
        .into_shape_with_order((rows, columns))
        .expect("calculated dimensions should always match data");
}

fn get_num_directly_accessible_rolls(rolls: &Array2<u8>) -> (Array2<u8>, u32) {
    // count all neighbors but do not count itself
    let kernel: Array2<u8> = array![
        [1, 1, 1], //
        [1, 0, 1], //
        [1, 1, 1], //
    ];
    // convolution is definitely not the most efficient option here because we only need results
    // for non-zero elements but I just felt like experimenting...
    let convolution = rolls
        .conv(&kernel, ConvMode::Same, PaddingMode::Zeros)
        .expect("convolution calculation should never fail because all dimensions are non-zero");
    let zipped = Zip::from(&convolution).and(rolls);
    let mut num_removed = 0;
    let remaining_rolls = zipped.map_collect(|&c, &i| {
        if i == 1 && c >= 4 {
            return 1;
        } else if i == 1 {
            num_removed += 1;
        }
        return 0;
    });
    return (remaining_rolls, num_removed);
}

fn get_num_accessible_rolls(rolls: Array2<u8>) -> u32 {
    let mut remaining_rolls: Array2<u8> = rolls;
    let mut num_accessible_rolls: u32 = 0;
    loop {
        let result = get_num_directly_accessible_rolls(&remaining_rolls);
        remaining_rolls = result.0;
        num_accessible_rolls += result.1;
        if result.1 == 0 {
            break;
        }
    }
    num_accessible_rolls
}

fn main() {
    let input = helpers::get_input(2025, 4).unwrap();
    let rolls = parse_input(&input);
    println!(
        "Number of directly accessible paper rolls: {}",
        get_num_directly_accessible_rolls(&rolls).1
    );
    println!(
        "Number of accessible paper rolls: {}",
        get_num_accessible_rolls(rolls)
    );
}
