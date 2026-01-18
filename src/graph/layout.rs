use std::collections::{HashMap, VecDeque};

use crate::graph::parser::{Graph, Node};


fn build_adjacency_graph(graph: &Graph) -> HashMap<String, Vec<String>> {
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();

    for node in &graph.nodes {
        adjacency.entry(node.clone()).or_insert_with(Vec::new);
    }

    for edge in &graph.edges {
        adjacency
            .entry(edge.from.clone())
            .or_insert_with(Vec::new)
            .push(edge.to.clone());
    }

    adjacency
}

fn assign_ranks(graph: &Graph, adjacency: &HashMap<Node, Vec<Node>>) -> HashMap<Node, usize> {
    let mut ranks = HashMap::new();

    let mut in_degrees = HashMap::new();
    for node in &graph.nodes {
        in_degrees.insert(node.clone(), 0);
    }
    for edge in &graph.edges {
        *in_degrees.get_mut(&edge.to).unwrap() += 1;
    }

    let mut queue = VecDeque::new();
    for (node, in_degree) in &in_degrees {
        if *in_degree == 0 {
            queue.push_back(node.clone());
        }
    }

    let mut current_rank = 0;
    while !queue.is_empty() {
        let current_layer_size = queue.len();

        for _ in 0..current_layer_size {
            let node = queue.pop_front().unwrap();

            for neighbor in adjacency.get(&node).unwrap() {
                let degree = in_degrees.get_mut(neighbor).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(neighbor.clone());
                }
            }

            ranks.insert(node, current_rank);
        }

        current_rank += 1;
    }

    ranks
}
