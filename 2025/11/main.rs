// https://adventofcode.com/2025/day/11

use itertools::Itertools;
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

static INITIAL_DEVICE: Device = Device::new("you");
static TARGET_DEVICE: Device = Device::new("out");

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
//  2. Exactly one input node path includes one or more desired nodes
//      ∴ visit count = only this input node visit count
//  3. More than one input node path includes the desired node
//      ∴ visit count = 0
// This works because each node is guaranteed to be unique so if
fn count_paths(device_inputs: DeviceInputAdjacencyList) -> u64 {
    let mut stack = vec![(TARGET_DEVICE)];
    let mut visit_counts: HashMap<Device, u64> = HashMap::new();
    while let Some(device) = stack.pop() {
        let inputs = device_inputs.get(&device).expect(&format!(
            "all devices should be in the adjacency list including {:?}",
            device
        ));

        if let Some(input_visit_counts) = inputs
            .iter()
            .map(|input| visit_counts.get(input))
            .fold_while(Some(0), |acc, curr| match curr {
                Some(current_visit_count) => {
                    itertools::FoldWhile::Continue(acc.map(|accumulated_visit_count| {
                        accumulated_visit_count + current_visit_count
                    }))
                }
                None => itertools::FoldWhile::Done(None),
            })
            .into_inner()
        {
            if device == INITIAL_DEVICE {
                visit_counts.insert(device, 1);
            } else {
                visit_counts.insert(device, input_visit_counts);
            }
        } else {
            let unresolved_inputs = inputs
                .iter()
                .filter(|device| !visit_counts.contains_key(device));
            stack.push(device);
            stack.extend(unresolved_inputs);
        }
    }
    return *visit_counts.get(&TARGET_DEVICE).expect("");
}

fn main() {
    let input = helpers::get_input(2025, 11).unwrap();
    let hashmap = parse_device_inputs(&input);
    println!(
        "The number of paths between 'you' and 'out' is: {}",
        count_paths(hashmap.clone())
    );
}
