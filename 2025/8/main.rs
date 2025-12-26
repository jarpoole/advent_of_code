// https://adventofcode.com/2025/day/8

use itertools::*;
use std::collections::{HashMap, HashSet};

#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
    index: usize,
}

impl JunctionBox {
    fn parse(s: &str, index: usize) -> Option<Self> {
        let mut coordinates = s.split(",");
        return Some(JunctionBox {
            x: coordinates.next()?.parse::<i64>().ok()?,
            y: coordinates.next()?.parse::<i64>().ok()?,
            z: coordinates.next()?.parse::<i64>().ok()?,
            index,
        });
    }

    fn euclidean_distance(&self, other: &JunctionBox) -> i64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)).isqrt()
    }
}

type AdjacencyList = HashMap<usize, Vec<usize>>;
type AdjacentJunctionBoxes = (JunctionBox, JunctionBox, i64);

fn get_all_graphs(adjacency_list: &AdjacencyList) -> Vec<HashSet<usize>> {
    let mut candidate_starting_vertices = adjacency_list.keys().map(|k| *k).collect::<HashSet<_>>();
    let mut graphs = Vec::new();
    while let Some(&starting_vertex) = candidate_starting_vertices.iter().next() {
        let mut stack = Vec::<usize>::from([starting_vertex]);
        let mut current_graph = HashSet::<usize>::new();
        while let Some(current_vertex) = stack.pop() {
            if !current_graph.contains(&current_vertex) {
                current_graph.insert(current_vertex);
                stack.extend(
                    adjacency_list
                        .get(&current_vertex)
                        .expect("all vertices should have an adjacency list pre-computed"),
                );
            }
        }
        candidate_starting_vertices.retain(|vertex| !current_graph.contains(vertex));
        graphs.push(current_graph);
    }
    return graphs;
}

fn junction_boxes(input: &str) -> impl Iterator<Item = JunctionBox> + Clone {
    input
        .split("\n")
        .filter_map(|line| (!line.is_empty()).then_some(line.trim()))
        .enumerate()
        .map(|(index, line)| {
            JunctionBox::parse(line, index).expect(&format!("should parse '{line}' successfully"))
        })
}

fn closest_junction_boxes(
    junction_boxes: impl Iterator<Item = JunctionBox> + Clone,
) -> Vec<AdjacentJunctionBoxes> {
    let mut edges = junction_boxes
        .tuple_combinations()
        .map(|(box_a, box_b)| (box_a, box_b, box_a.euclidean_distance(&box_b)))
        .collect::<Vec<_>>();
    edges.sort_by(|a, b| a.2.cmp(&b.2)); // sort is ascending by default
    return edges;
}

fn connect_junction_boxes(adjacency_list: &mut AdjacencyList, edge: &AdjacentJunctionBoxes) {
    let vertex_a = adjacency_list.entry(edge.0.index).or_insert(Vec::new());
    vertex_a.push(edge.1.index);
    let vertex_b = adjacency_list.entry(edge.1.index).or_insert(Vec::new());
    vertex_b.push(edge.0.index);
}

fn get_fully_connecting_edge<'a>(
    mut edges: impl Iterator<Item = &'a AdjacentJunctionBoxes>,
    vertices: impl Iterator<Item = JunctionBox>,
) -> AdjacentJunctionBoxes {
    let mut adjacency_list: AdjacencyList =
        HashMap::from_iter(vertices.map(|vertex| (vertex.index, Vec::new())));
    // Improve performance by approximately 15x by only checking graph connectivity
    // if it is actually possible that the vertices could be fully connected.
    // Before the first edge, every vertex is guaranteed to be in a separate graph
    // so start with this as an estimate.
    let mut potential_num_graphs = adjacency_list.len() - 1;
    let junction_boxes = edges
        .find(|edge| {
            connect_junction_boxes(&mut adjacency_list, edge);
            potential_num_graphs -= 1; // decrease estimate for every edge added
            if potential_num_graphs <= 1 {
                let num_graphs = get_all_graphs(&adjacency_list).len();
                // if the vertices are still not fully connected, update the
                // estimate based on the actual minimum remaining number of edges
                // that would be required
                potential_num_graphs = num_graphs - 1;
                return num_graphs == 1;
            }
            return false;
        })
        .expect(
            "based on part2 description all junction boxes will become connected at some point",
        );
    return *junction_boxes;
}

fn get_multiplied_size_of_largest_circuits<'a>(
    edges: impl Iterator<Item = &'a AdjacentJunctionBoxes>,
) -> usize {
    let adjacency_list = edges.fold(HashMap::<usize, Vec<usize>>::new(), |mut hashmap, edge| {
        connect_junction_boxes(&mut hashmap, edge);
        hashmap
    });

    let mut graphs = get_all_graphs(&adjacency_list);
    graphs.sort_by(|a, b| a.len().cmp(&b.len()));
    let size_of_largest_graphs: usize = graphs
        .iter()
        .rev()
        .take(3)
        .map(|g| g.len())
        .fold(1, |acc, curr| acc * curr);
    return size_of_largest_graphs;
}

fn get_multiplied_fully_connecting_x_coordinates<'a>(
    edges: impl Iterator<Item = &'a AdjacentJunctionBoxes>,
    vertices: impl Iterator<Item = JunctionBox>,
) -> i64 {
    let fully_connecting_edge = get_fully_connecting_edge(edges, vertices);
    fully_connecting_edge.0.x * fully_connecting_edge.1.x
}

fn main() {
    let input = helpers::get_input(2025, 8).unwrap();
    let junction_boxes = junction_boxes(&input);
    let closest_junction_boxes = closest_junction_boxes(junction_boxes.clone());
    println!(
        "The number of junction boxes in the three largest circuits multiplied is: {}",
        get_multiplied_size_of_largest_circuits(closest_junction_boxes.iter().take(1000))
    );
    println!(
        "The multiplied connecting junction box X coordinates are: {}",
        get_multiplied_fully_connecting_x_coordinates(
            closest_junction_boxes.iter(),
            junction_boxes
        )
    )
}
