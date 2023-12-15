/// Module: graph::ged
/// Mapping Nodes: The function ged_recursive tries different ways to map nodes from graph1 to graph2, including substitutions and deletions.
/// Calculating Distance: For each mapping, calculate_edit_distance calculates the edit distance based on the number of substitutions, deletions, and additions required.
/// Performance: This algorithm has exponential time complexity and is not suitable for large graphs.
/// Simplifications: This implementation does not consider edges and assumes all nodes are equivalent. A complete GED algorithm would need to consider edge differences and potentially unique node and edge attributes.
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;
use std::collections::HashSet;

// Define the cost of operations
const COST_ADD: usize = 1;
const COST_DEL: usize = 1;
const COST_SUB: usize = 1;
const COST_ADD_DEL: usize = 1;

fn ged_recursive(
    graph1: &DiGraph<(), ()>,
    graph2: &DiGraph<(), ()>,
    mapping: &mut [(Option<NodeIndex>, Option<NodeIndex>)],
    index: usize,
) -> usize {
    if index == mapping.len() {
        // All nodes are processed, calculate the edit distance for this mapping
        return calculate_edit_distance(graph1, graph2, mapping);
    }

    let mut min_distance = usize::MAX;

    // Option 1: Map the current node in graph1 to a node in graph2
    for i in 0..graph2.node_count() {
        let node2 = NodeIndex::new(i);
        if !mapping.iter().any(|(_, b)| *b == Some(node2)) {
            mapping[index] = (Some(NodeIndex::new(index)), Some(node2));
            min_distance = min_distance.min(ged_recursive(graph1, graph2, mapping, index + 1));
        }
    }

    // Option 2: Delete the current node in graph1 (map it to None in graph2)
    mapping[index] = (Some(NodeIndex::new(index)), None);
    min_distance = min_distance.min(ged_recursive(graph1, graph2, mapping, index + 1));

    // Reset the mapping for the next iteration
    mapping[index] = (None, None);
    min_distance
}

fn calculate_edit_distance(
    graph1: &DiGraph<(), ()>,
    graph2: &DiGraph<(), ()>,
    mapping: &[(Option<NodeIndex>, Option<NodeIndex>)],
) -> usize {
    let mut edit_distance = 0;

    let mapped_nodes1: HashSet<_> = mapping.iter().filter_map(|(a, _)| *a).collect();
    let mapped_nodes2: HashSet<_> = mapping.iter().filter_map(|(_, b)| *b).collect();

    // Cost of node substitutions and deletions
    for (a, b) in mapping {
        match (a, b) {
            (Some(_), Some(_)) => edit_distance += COST_SUB,
            (Some(_), None) => edit_distance += COST_ADD_DEL,
            _ => (),
        }
    }

    // Cost of node additions
    edit_distance += graph2.node_count() - mapped_nodes2.len();

    edit_distance
}

fn graph_edit_distance(graph1: &DiGraph<(), ()>, graph2: &DiGraph<(), ()>) -> usize {
    let node_count = graph1.node_count().max(graph2.node_count());
    let mut mapping = vec![(None, None); node_count];
    ged_recursive(graph1, graph2, &mut mapping, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ged() {
        let mut graph1 = DiGraph::new();
        let mut graph2 = DiGraph::new();

        let gn1 = graph1.add_node(());
        let gn2 = graph1.add_node(());

        let gn3 = graph1.add_node(());
        let gn4 = graph1.add_node(());

        graph1.extend_with_edges([(gn1, gn2), (gn1, gn3), (gn3, gn4)]);

        let gn5 = graph2.add_node(());
        let gn6 = graph2.add_node(());
        let gn7 = graph2.add_node(());
        let gn8 = graph2.add_node(());

        graph2.extend_with_edges([(gn5, gn6), (gn6, gn7), (gn7, gn8)]);

        // Example: Add nodes to graph1 and graph2
        // ...

        let distance = graph_edit_distance(&graph1, &graph2);
        println!("Graph Edit Distance: {}", distance);
    }
}
