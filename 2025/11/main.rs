// https://adventofcode.com/2025/day/11

use itertools::{FoldWhile, Itertools};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Device<'a>(&'a str);

impl<'a> Device<'a> {
    const fn new(name: &'a str) -> Self {
        Device(name)
    }
}

struct Constraints<'a> {
    from: Device<'a>,
    to: Device<'a>,
    including: HashSet<Device<'a>>,
}

type DeviceInputAdjacencyList<'a> = HashMap<Device<'a>, Vec<Device<'a>>>;

fn parse_device_inputs(input: &str) -> DeviceInputAdjacencyList<'_> {
    let mut hashmap: DeviceInputAdjacencyList = HashMap::new();
    input
        .split("\n")
        .filter_map(|line| (!line.is_empty()).then_some(line.trim()))
        .for_each(|line| {
            let mut iter = line.split(":");
            let device = Device::new(iter.next().expect(""));
            hashmap.entry(device).or_insert(Vec::new());
            iter.next()
                .expect("")
                .split_whitespace()
                .for_each(|output| {
                    let output_device = Device::new(output);
                    hashmap
                        .entry(output_device)
                        .and_modify(|entry| entry.push(device))
                        .or_insert(vec![device]);
                });
        });
    return hashmap;
}

// When determining the visit count for any given node there are several possibilities
//  1. No input node paths include any of the desired nodes
//      ∴ visit count = sum all input visit counts
//  2. Exactly one input node path includes one or more desired nodes but not all of them
//      ∴ visit count = only this input node visit count
//  3. More than one input node path includes some of the desired nodes but not all of them
//      ∴ visit count = 0
//  4. More than one input node path includes some of the desired nodes and some of the input
//     include all of them
//      ∴ visit count = only the input paths that include all the desired nodes
// This works because each node is guaranteed to be unique so if different required nodes
// appear in separate branches and the graph is acyclic then no path can contain both
#[derive(PartialEq)]
enum InputVisitState<'a> {
    MissingDependencies,
    AllConstraintsSatisfied(u64),
    SomeConstraintsSatisfied(u64, HashSet<Device<'a>>),
    NoConstraintsSatisfied(u64),
    ConflictingConstraints,
}
fn count_paths(device_inputs: DeviceInputAdjacencyList, constraints: Constraints) -> u64 {
    let mut stack = vec![constraints.to];
    let mut visit_counts: HashMap<Device, (u64, Option<HashSet<Device>>)> = HashMap::new();
    while let Some(device) = stack.pop() {
        let inputs = device_inputs.get(&device).expect(&format!(
            "all devices should be in the adjacency list including {:?}",
            device
        ));

        if device == constraints.from {
            visit_counts.insert(device, (1, None));
            continue;
        }

        let visit_count = inputs
            .iter()
            .map(|input| visit_counts.get(input))
            .fold_while(
                InputVisitState::NoConstraintsSatisfied(0),
                |acc, curr| match curr {
                    Some((current_input_visit_count, v)) => {
                        match acc {
                            // check if one or more input paths already satisfy all constraints
                            InputVisitState::AllConstraintsSatisfied(accumulated_visit_count) => {
                                if let Some(current_visited_constraints) = v
                                    && *current_visited_constraints == constraints.including
                                {
                                    // more than one input has paths which already fully satisfy constraints
                                    // so sum them
                                    return FoldWhile::Continue(
                                        InputVisitState::AllConstraintsSatisfied(
                                            accumulated_visit_count + current_input_visit_count,
                                        ),
                                    );
                                } else {
                                    // discard the result if the current input paths
                                    return FoldWhile::Continue(acc);
                                }
                            }
                            // check if one or more input paths satisfy some of the constraints
                            InputVisitState::SomeConstraintsSatisfied(
                                accumulated_visit_count,
                                accumulated_visited_constraints,
                            ) => {
                                if let Some(current_visited_constraints) = v
                                    && *current_visited_constraints == constraints.including
                                {
                                    return FoldWhile::Continue(
                                        InputVisitState::AllConstraintsSatisfied(
                                            *current_input_visit_count,
                                        ),
                                    );
                                } else if let Some(current_visited_constraints) = v
                                    && *current_visited_constraints
                                        == accumulated_visited_constraints
                                {
                                    return FoldWhile::Continue(
                                        InputVisitState::SomeConstraintsSatisfied(
                                            accumulated_visit_count + current_input_visit_count,
                                            accumulated_visited_constraints,
                                        ),
                                    );
                                // if multiple paths each have different partial constraints then neither can
                                // ever be completed
                                } else if let Some(current_visited_constraints) = v
                                    && *current_visited_constraints
                                        != accumulated_visited_constraints
                                {
                                    return FoldWhile::Continue(
                                        InputVisitState::ConflictingConstraints,
                                    );
                                // discard the current input as it can never be complete given it is missing
                                // at least one constraint other input paths have already passed through
                                } else {
                                    return FoldWhile::Continue(
                                        InputVisitState::SomeConstraintsSatisfied(
                                            accumulated_visit_count,
                                            accumulated_visited_constraints.clone(),
                                        ),
                                    );
                                }
                            }
                            InputVisitState::NoConstraintsSatisfied(accumulated_visit_count) => {
                                if let Some(current_visited_constraints) = v
                                    && *current_visited_constraints == constraints.including
                                {
                                    return FoldWhile::Continue(
                                        InputVisitState::AllConstraintsSatisfied(
                                            *current_input_visit_count,
                                        ),
                                    );
                                } else if let Some(current_visited_constraints) = v {
                                    return FoldWhile::Continue(
                                        InputVisitState::SomeConstraintsSatisfied(
                                            *current_input_visit_count,
                                            current_visited_constraints.clone(),
                                        ),
                                    );
                                } else {
                                    return FoldWhile::Continue(
                                        InputVisitState::NoConstraintsSatisfied(
                                            *current_input_visit_count + accumulated_visit_count,
                                        ),
                                    );
                                }
                            }
                            InputVisitState::ConflictingConstraints => FoldWhile::Continue(acc),
                            InputVisitState::MissingDependencies => {
                                unimplemented!("fold while will always returns early")
                            }
                        }
                    }
                    None => FoldWhile::Done(InputVisitState::MissingDependencies),
                },
            )
            .into_inner();

        match visit_count {
            InputVisitState::AllConstraintsSatisfied(input_visit_counts) => {
                visit_counts.insert(
                    device,
                    (input_visit_counts, Some(constraints.including.clone())),
                );
            }
            InputVisitState::SomeConstraintsSatisfied(
                input_visit_counts,
                mut visited_constraints,
            ) => {
                if constraints.including.contains(&device) {
                    visited_constraints.insert(device);
                }
                visit_counts.insert(device, (input_visit_counts, Some(visited_constraints)));
            }
            InputVisitState::NoConstraintsSatisfied(input_visit_counts) => {
                if constraints.including.contains(&device) {
                    visit_counts
                        .insert(device, (input_visit_counts, Some(HashSet::from([device]))));
                } else {
                    visit_counts.insert(device, (input_visit_counts, None));
                }
            }
            InputVisitState::ConflictingConstraints => {
                visit_counts.insert(device, (0, None));
            }
            InputVisitState::MissingDependencies => {
                let unresolved_inputs = inputs
                    .iter()
                    .filter(|device| !visit_counts.contains_key(device));
                stack.push(device);
                stack.extend(unresolved_inputs);
            }
        }
        if device == Device::new("fff") {
            dbg!(&visit_counts);
        }
    }
    return (*visit_counts.get(&constraints.to).expect("")).0;
}

fn count_part1_paths(device_inputs: DeviceInputAdjacencyList) -> u64 {
    count_paths(
        device_inputs,
        Constraints {
            from: Device::new("you"),
            to: Device::new("out"),
            including: HashSet::new(),
        },
    )
}

fn count_part2_paths(device_inputs: DeviceInputAdjacencyList) -> u64 {
    count_paths(
        device_inputs,
        Constraints {
            from: Device::new("svr"),
            to: Device::new("out"),
            including: HashSet::from([Device::new("fft"), Device::new("dac")]),
        },
    )
}

fn main() {
    let input = helpers::get_input(2025, 11).unwrap();
    let hashmap = parse_device_inputs(&input);
    println!(
        "The number of paths between 'you' and 'out' is: {}",
        count_part1_paths(hashmap.clone())
    );
    println!(
        "The number of paths between 'svr' and 'out' which pass through 'dac' and 'fft' is: {}",
        count_part2_paths(hashmap)
    );
}
