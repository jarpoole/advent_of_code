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

type DeviceInputAdjacencyList<'a> = HashMap<Device<'a>, Vec<Device<'a>>>;

fn parse_device_inputs(input: &str) -> DeviceInputAdjacencyList<'_> {
    let mut hashmap: DeviceInputAdjacencyList = HashMap::new();
    input
        .split("\n")
        .filter_map(|line| (!line.is_empty()).then_some(line.trim()))
        .for_each(|line| {
            let mut iter = line.split(":");
            let device = Device::new(
                iter.next()
                    .expect("the device name should always appear before the colon"),
            );
            hashmap.entry(device).or_insert(Vec::new());
            iter.next()
                .expect(
                    "split should always return an empty string at a minimum after the delimiter",
                )
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

struct Constraints<'a> {
    /// starting device in path
    from: Device<'a>,
    /// final device in path
    to: Device<'a>,
    /// devices that must appear in any valid path
    including: HashSet<Device<'a>>,
}

#[derive(PartialEq)]
enum State<'a> {
    /// The number of paths to the current device (vertex) cannot be calculated yet
    /// because the number of paths has not been calculated for at least one input
    MissingDependencies,
    /// The current device (vertex) has already been shown to have at least one path
    /// which includes all the required intermediate devices constraints
    AllConstraintsSatisfied(u64),
    /// The current device (vertex) has been shown to have at least one path which
    /// includes one or more of the intermediate devices constraints
    SomeConstraintsSatisfied(u64, HashSet<Device<'a>>),
    /// The current device (vertex) has not been show to be reachable by any paths
    /// including any of the required device constraints yet
    NoConstraintsSatisfied(u64),
    /// The current device (vertex) has multiple input paths which each have been shown
    /// to include different device constraints. Thus no single path can ever include all
    /// the required device constraints
    ConflictingConstraints,
}

struct Visit<'a> {
    count: u64,
    visited_constraints: Option<HashSet<Device<'a>>>,
}

fn count_paths(device_inputs: DeviceInputAdjacencyList, constraints: Constraints) -> u64 {
    let mut stack = vec![constraints.to];
    let mut visits: HashMap<Device, Visit> = HashMap::new();
    while let Some(device) = stack.pop() {
        let inputs = device_inputs.get(&device).expect(&format!(
            "all devices should be in the adjacency list including {:?}",
            device
        ));

        if device == constraints.from {
            visits.insert(
                device,
                Visit {
                    count: 1,
                    visited_constraints: None,
                },
            );
            continue;
        }

        // The key idea when determining the visit count for a vertex is that because each vertex
        // is guaranteed to be unique and the graph is acyclic, if different required constraints
        // appear in separate input paths and the graph is acyclic, no path can contain both and
        // therefore neither path should be counted
        //
        // Possibilities when analyzing the inputs for a vertex
        //  1. No input vertex paths include any of the desired vertices
        //      ∴ visit count = sum all input visit vertices
        //  2. One or more input vertex path includes one or more of the SAME desired vertex but
        //     not all of them
        //      ∴ visit count = sum of only these input vertices
        //  3. More than one input vertex path includes DIFFERENT desired vertices but not all of them
        //      ∴ visit count = 0
        //  4. More than one input vertex path includes some of the desired vertices
        //     and some of the input include all of them
        //      ∴ visit count = sum of only the input paths that include all the desired vertices
        let visit_count = inputs
            .iter()
            .map(|input| visits.get(input))
            .fold_while(State::NoConstraintsSatisfied(0), |acc, curr| match curr {
                Some(current_input_visit) => {
                    match acc {
                        // check if one or more input paths already satisfy all constraints
                        State::AllConstraintsSatisfied(accumulated_visit_count) => {
                            if let Some(current_visited_constraints) =
                                &current_input_visit.visited_constraints
                                && *current_visited_constraints == constraints.including
                            {
                                // more than one input has paths which already fully satisfy constraints
                                // so sum them
                                return FoldWhile::Continue(State::AllConstraintsSatisfied(
                                    accumulated_visit_count + current_input_visit.count,
                                ));
                            } else {
                                // otherwise discard the paths from this input because they can
                                // never include all the required constraints
                                return FoldWhile::Continue(acc);
                            }
                        }
                        // check if one or more input paths satisfy some of the constraints
                        State::SomeConstraintsSatisfied(
                            accumulated_visit_count,
                            accumulated_visited_constraints,
                        ) => {
                            // if an input satisfies all constraints then we no longer need to consider
                            // any other input which does not also already satisfy all constraints
                            if let Some(current_visited_constraints) =
                                &current_input_visit.visited_constraints
                                && *current_visited_constraints == constraints.including
                            {
                                return FoldWhile::Continue(State::AllConstraintsSatisfied(
                                    current_input_visit.count,
                                ));
                            // if multiple paths each include the same partial constraints then sum
                            } else if let Some(current_visited_constraints) =
                                &current_input_visit.visited_constraints
                                && *current_visited_constraints == accumulated_visited_constraints
                            {
                                return FoldWhile::Continue(State::SomeConstraintsSatisfied(
                                    accumulated_visit_count + current_input_visit.count,
                                    accumulated_visited_constraints,
                                ));
                            // if multiple paths each have different partial constraints then neither can
                            // ever be completed
                            } else if let Some(current_visited_constraints) =
                                &current_input_visit.visited_constraints
                                && *current_visited_constraints != accumulated_visited_constraints
                            {
                                return FoldWhile::Continue(State::ConflictingConstraints);
                            // discard the current input as it can never be complete given it is missing
                            // at least one constraint other input paths have already passed through
                            } else {
                                return FoldWhile::Continue(State::SomeConstraintsSatisfied(
                                    accumulated_visit_count,
                                    accumulated_visited_constraints.clone(),
                                ));
                            }
                        }
                        // check if none of the input paths include any of the constraint vertices yet
                        State::NoConstraintsSatisfied(accumulated_visit_count) => {
                            // if an input satisfies all constraints then we no longer need to consider
                            // any other input which does not also already satisfy all constraints
                            if let Some(current_visited_constraints) =
                                &current_input_visit.visited_constraints
                                && *current_visited_constraints == constraints.including
                            {
                                return FoldWhile::Continue(State::AllConstraintsSatisfied(
                                    current_input_visit.count,
                                ));
                            // if this is the first input whose paths include some but not all constraint
                            // vertices, then switch states to ensure that if this occurs again for future
                            // input we can check if they are the same constraint vertices and can be summed
                            // of if they are different and everything needs to be discarded
                            } else if let Some(current_visited_constraints) =
                                &current_input_visit.visited_constraints
                            {
                                return FoldWhile::Continue(State::SomeConstraintsSatisfied(
                                    current_input_visit.count,
                                    current_visited_constraints.clone(),
                                ));
                                // otherwise because no input path includes any constraint vertices yet,
                                // all input paths are still potentially valid so sum them
                            } else {
                                return FoldWhile::Continue(State::NoConstraintsSatisfied(
                                    current_input_visit.count + accumulated_visit_count,
                                ));
                            }
                        }
                        State::ConflictingConstraints => FoldWhile::Continue(acc),
                        State::MissingDependencies => {
                            unreachable!("fold-while will always returns early")
                        }
                    }
                }
                None => FoldWhile::Done(State::MissingDependencies),
            })
            .into_inner();

        match visit_count {
            State::AllConstraintsSatisfied(input_visit_counts) => {
                visits.insert(
                    device,
                    Visit {
                        count: input_visit_counts,
                        visited_constraints: Some(constraints.including.clone()),
                    },
                );
            }
            State::SomeConstraintsSatisfied(input_visit_counts, mut visited_constraints) => {
                if constraints.including.contains(&device) {
                    visited_constraints.insert(device);
                }
                visits.insert(
                    device,
                    Visit {
                        count: input_visit_counts,
                        visited_constraints: Some(visited_constraints),
                    },
                );
            }
            State::NoConstraintsSatisfied(input_visit_counts) => {
                if constraints.including.contains(&device) {
                    visits.insert(
                        device,
                        Visit {
                            count: input_visit_counts,
                            visited_constraints: Some(HashSet::from([device])),
                        },
                    );
                } else {
                    visits.insert(
                        device,
                        Visit {
                            count: input_visit_counts,
                            visited_constraints: None,
                        },
                    );
                }
            }
            State::ConflictingConstraints => {
                visits.insert(
                    device,
                    Visit {
                        count: 0,
                        visited_constraints: None,
                    },
                );
            }
            State::MissingDependencies => {
                // to guarantee that the dependencies are resolved next time this device is
                // considered, we leverage a pre-order graph traversal. Given we are using a
                // stack, this means we need to push the input dependent vertices on last so they
                // will be considered first
                let unresolved_inputs = inputs.iter().filter(|device| !visits.contains_key(device));
                stack.push(device);
                stack.extend(unresolved_inputs);
            }
        }
    }
    return visits
        .get(&constraints.to)
        .map(|visit| visit.count)
        .unwrap_or(0);
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
