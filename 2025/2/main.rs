use std::collections::HashSet;

#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

struct Range {
    lower: u64,
    upper: u64,
}

type InvalidIdFn = fn(&str) -> bool;

impl Range {
    fn new(str: &str) -> Option<Self> {
        // Note that we must trim off
        let mut bounds = str.trim().split("-").map(|bound| bound.parse::<u64>().ok());
        Option::zip(bounds.next().flatten(), bounds.next().flatten()).map(|b| Range {
            lower: b.0,
            upper: b.1,
        })
    }
    fn sum_invalid_ids(self, invalid_id_fn: InvalidIdFn, set: &mut HashSet<u64>) -> u64 {
        (self.lower..=self.upper)
            .filter_map(|id| {
                (!set.contains(&id) && invalid_id_fn(&id.to_string())).then(|| {
                    set.insert(id);
                    id
                })
            })
            .sum()
    }
}

fn find_invalid_ids(input: &str, invalid_id_fn: InvalidIdFn) -> u64 {
    let mut set = HashSet::new(); // protect against overlapping ranges
    input
        .split(",")
        .map(|range| {
            Range::new(range)
                .map(|range| range.sum_invalid_ids(invalid_id_fn, &mut set))
                .expect(format!("failed to parse '{}'", range).as_str())
        })
        .sum()
}

fn is_invalid_id_part1(id: &str) -> bool {
    if !id.len().is_multiple_of(2) {
        return false;
    }
    let halves = id.split_at(id.len() / 2);
    return halves.0 == halves.1;
}

fn is_invalid_id_part2(id: &str) -> bool {
    if id.len() < 2 {
        return false;
    }
    for group_size in 1..=(id.len() / 2) {
        let mut chunks = id.as_bytes().chunks(group_size);
        let first_chunk = chunks.next();
        if chunks.all(|chunk| Some(chunk) == first_chunk) {
            return true;
        }
    }
    return false;
}

fn main() {
    let input = helpers::get_input(2025, 2).unwrap();
    println!(
        "The sum of all invalid ids in part 1 is: {}",
        find_invalid_ids(&input, is_invalid_id_part1)
    );
    println!(
        "The sum of all invalid ids in part 2 is: {}",
        find_invalid_ids(&input, is_invalid_id_part2)
    );
}
//mine:    30962646601
//correct: 30962646823
