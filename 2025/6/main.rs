use std::{borrow::Borrow, str::Chars};

use ndarray::{Array2, ArrayBase};

#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn new(operator: char) -> Option<Self> {
        match operator {
            '+' => Some(Self::Add),
            '*' => Some(Self::Multiply),
            _ => None,
        }
    }

    fn apply<'a, T: IntoIterator<Item = impl Borrow<u64>>>(&self, numbers: T) -> u64 {
        match self {
            Self::Add => numbers.into_iter().fold(0, |mut accumulator, number| {
                accumulator += number.borrow();
                accumulator
            }),
            Self::Multiply => numbers.into_iter().fold(1, |mut accumulator, number| {
                accumulator *= number.borrow();
                accumulator
            }),
        }
    }
}

struct ProblemIterator<'a> {
    row_iterators: Vec<Chars<'a>>,
}

impl<'a> ProblemIterator<'a> {
    fn new(row_iterators: Vec<Chars<'a>>) -> Self {
        ProblemIterator { row_iterators }
    }
}

impl<'a> Iterator for ProblemIterator<'a> {
    type Item = u64;

    /// Produces the answer to a single problem at a time from left to right
    fn next(&mut self) -> Option<u64> {
        // parse the operator from the last row in the input. To avoid making an
        // assumption about where the operator is located within the problem,
        // simply consume whitespace until one is found
        let operator = Operator::new(
            self.row_iterators
                .last_mut()?
                .filter(|c| !c.is_whitespace())
                .next()?,
        )?;

        return Some(operator.apply(std::iter::from_fn(|| {
            let next: u64 = self
                .row_iterators
                .iter_mut()
                // parse the digits of each number in reverse (from the
                // bottom) so that an incrementing index matches the place
                // value of digit when raised to the power of ten
                .rev()
                // skip the operator row
                .skip(1)
                // ignore empty rows when parsing each digit because
                // not all numbers in each column have a digit in all
                // rows
                .filter_map(|row| match row.next()? {
                    ' ' => None,
                    c => Some(c.to_digit(10)? as u64),
                })
                // use zip instead of .enumerate() to set the index type
                // to u32 instead of usize
                .zip(0u32..)
                .map(|(digit, i)| digit * 10u64.pow(i))
                .sum();
            // each problem is delineated by a completely empty column
            // which, when parsed, will become 0. Therefore, when this
            // occurs we should stop iteration by returning None and
            // apply the previously parsed operator
            return (next > 0).then_some(next);
        })));
    }
}

fn part1(input: &str) -> u64 {
    let mut num_rows = 0;
    let mut rows = input
        .split('\n')
        .filter_map(|row| (!row.is_empty()).then_some(row.trim()))
        .rev();

    let operators = rows
        .next()
        .expect("the last non-empty row should always be operators (due to the reverse operator)")
        .split_ascii_whitespace()
        .map(|operator_str| {
            let operator = operator_str
                .chars()
                .next()
                .expect("operator should always be a single character");
            Operator::new(operator).expect("should support all operators")
        });

    let numbers_vec = rows
        .map(|row| {
            num_rows += 1;
            row.split_ascii_whitespace().map(|number| {
                number
                    .parse::<u64>()
                    .expect("non whitespace characters should always form valid integers because the operators were already removed above")
            })
        })
        .flatten()
        .collect::<Vec<u64>>();
    let num_columns = numbers_vec.len() / num_rows;
    // It is not wasteful to collect into a vec above because
    // `ArrayBase::from_vec` does not reallocate here
    let numbers: Array2<u64> = ArrayBase::from_vec(numbers_vec)
        .into_shape_with_order((num_rows, num_columns))
        .expect("input should always be square");

    return Iterator::zip(numbers.columns().into_iter(), operators)
        .map(|(column, operator)| operator.apply(column))
        .sum();
}

fn part2(input: &str) -> u64 {
    let row_iterators: Vec<_> = input
        .split('\n')
        .filter_map(|row| (!row.is_empty()).then_some(row.chars()))
        .collect();
    ProblemIterator::new(row_iterators).sum()
}

fn main() {
    let input = helpers::get_input(2025, 6).unwrap();

    println!("The part 1 grand total is: {}", part1(&input));
    println!("The part 2 grand total is: {}", part2(&input));
}
