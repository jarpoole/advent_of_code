#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

fn get_max_digit(slice: &str) -> Option<(usize, char)> {
    slice
        .chars()
        // "max_by_key" returns the last element in case of ties and we need the first
        // one so search from back to front instead
        .rev()
        .enumerate()
        .max_by_key(|d| d.1)
        // Subtract the found index from the length to account for the fact that we searched
        // in reverse
        .map(|d| (slice.len() - d.0, d.1))
}

fn get_max_bank_joltage(bank: &str, num_cells: u8) -> u64 {
    // preallocate because we know how many digits are in the joltage
    let mut buf = String::with_capacity(num_cells.into());
    let mut slice = bank;
    // iterate backwards so that we can use the index "reserved_cells" to track how
    // many digits need to be reserved for future choices to ensure we select exactly
    // "num_cells" digits
    for reserved_cells in (0..num_cells as usize).rev() {
        // do not consider reserved digits when selecting
        let result = get_max_digit(&slice[0..(slice.len() - reserved_cells)])
            .expect("should always be at least one digit and therefore a maximum");
        // update remaining digits to be only those to the right of the selected
        // maximum digit
        slice = &slice[result.0..slice.len()];
        buf.push(result.1);
    }
    return buf
        .parse::<u64>()
        .expect("there should not be any non-digit characters in each input line");
}

fn get_max_joltage(input: &str, num_cells: u8) -> u64 {
    input
        // one bank per line
        .split("\n")
        // discard empty lines
        .map(|bank| bank.trim())
        .filter(|bank| !bank.is_empty())
        // compute sum
        .map(|bank| get_max_bank_joltage(bank, num_cells))
        .sum()
}

fn main() {
    let input = helpers::get_input(2025, 3).unwrap();
    println!(
        "Largest joltage using 2 cells: {}",
        get_max_joltage(&input, 2)
    );
    println!(
        "Largest joltage using 12 cells: {}",
        get_max_joltage(&input, 12)
    );
}
