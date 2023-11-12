// https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html#method.node_weight

use super::data::NLGraph;

use log::warn;
use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use petgraph::visit::IntoNeighborsDirected;
use petgraph::visit::Visitable;
use petgraph::Direction;

pub trait GraphAnalysis {
    fn node_degree(&mut self);

    fn is_weakly_connected(&self) -> bool;

    fn density(&self) -> f64;
    fn node_count(&self) -> usize;
    fn edge_count(&self) -> usize;

    fn walk(&self);

    // centrality measures
    // Degree centrality is a way of measuring the importance of a node within a graph.
    //  It is based on the number of links incident upon a node (i.e., the number of ties that a node has). For directed graphs, you can compute two types of degree centrality: in-degree centrality and out-degree centrality.
    fn degree_centrality(&mut self);

    // Closeness centrality measures how close a node is to all other nodes in the network.
    //  It is calculated as the reciprocal of the sum of the length of the shortest paths between the node and all other nodes in the graph.
    //  Thus, the more central a node is, the closer it is to all other nodes.
    fn closeness_centrality(&mut self);

    // fn eigenvector_centrality(&mut self, tolerance: f64, max_iter: usize);

    // // local clustering measures
    fn local_clustering_coefficient(&mut self);

    fn is_cyclic_directed(&self) -> bool;

    fn vis(&self);

    fn to_cyjson(&self) -> String;
}

impl GraphAnalysis for NLGraph {
    fn node_degree(&mut self) {
        for ind in self.node_indices() {
            let indegree = self.neighbors_directed(ind, petgraph::Incoming).count();
            let outdegree = self.neighbors_directed(ind, petgraph::Outgoing).count();

            let node = self.node_weight_mut(ind).unwrap();
            node.indegree = indegree;
            node.outdegree = outdegree;
        }
    }

    fn is_weakly_connected(&self) -> bool {
        let mut visited = vec![false; self.node_count()];
        let mut stack = vec![];
        let node_index = self.node_indices().next().unwrap();
        stack.push(node_index.index());
        visited[node_index.index()] = true;
        while stack.pop().is_some() {
            for neighbor in self.neighbors(node_index) {
                if !visited[neighbor.index()] {
                    visited[neighbor.index()] = true;
                    stack.push(neighbor.index());
                }
            }
        }
        visited.len() == self.node_count()
    }

    fn density(&self) -> f64 {
        let n = self.node_count();
        let m = self.edge_count();
        2.0 * m as f64 / (n * (n - 1)) as f64
    }

    fn node_count(&self) -> usize {
        self.node_count()
    }

    fn edge_count(&self) -> usize {
        self.edge_count()
    }

    fn walk(&self) {
        let mut paths = Vec::new(); // To hold all paths
        let start_nodes = self.externals(Direction::Incoming).collect::<Vec<_>>(); // Nodes with in-degree == 0

        // Recursive DFS function
        fn dfs_visit<NLGraph>(
            graph: &NLGraph,
            node: NodeIndex,
            current_path: &mut Vec<NodeIndex>,
            paths: &mut Vec<Vec<NodeIndex>>,
        ) where
            NLGraph: IntoNeighborsDirected + Visitable<NodeId = NodeIndex>,
        {
            // Visit the current node
            current_path.push(node);

            // Check if it's an ending node (out-degree == 0)
            if graph.neighbors_directed(node, Direction::Outgoing).count() == 0 {
                // Save the path
                paths.push(current_path.clone());
            } else {
                // Continue with unvisited neighbors
                for neighbor in graph.neighbors_directed(node, Direction::Outgoing) {
                    dfs_visit(graph, neighbor, current_path, paths);
                }
            }

            // Backtrack
            current_path.pop();
        }

        // Start the DFS from each starting node
        for &start_node in &start_nodes {
            let mut current_path = Vec::new();
            dfs_visit(&self, start_node, &mut current_path, &mut paths);
        }

        // Output all paths
        for path in &paths {
            println!("{:?}", path);
        }
    }

    fn degree_centrality(&mut self) {
        // The normalization factor is based on the number of nodes minus 1
        // It's the maximum possible degree of any node
        let normalization = (self.node_count() - 1) as f32;

        // Iterate over all nodes to compute in-degree and out-degree centrality
        for node_ind in self.node_indices() {
            // Compute in-degree and out-degree for the node
            let in_degree = self
                .neighbors_directed(node_ind, Direction::Incoming)
                .count() as f32;
            let out_degree = self
                .neighbors_directed(node_ind, Direction::Outgoing)
                .count() as f32;

            let node_data = self.node_weight_mut(node_ind).unwrap();
            node_data.in_degree_centrality = in_degree / normalization;
            node_data.out_degree_centrality = out_degree / normalization;
        }
    }

    fn closeness_centrality(&mut self) {
        if !self.is_weakly_connected() {
            warn!("Graph is not weakly connected, skipping closeness centrality calculation");
            return;
        }

        let node_count = self.node_count() as f32;

        // Compute shortest paths from each node to all other nodes
        for node_ind in self.node_indices() {
            let shortest_paths = dijkstra(&*self, node_ind, None, |_| 1);

            // Calculate the sum of the shortest paths to all other nodes
            let total_distance: usize = shortest_paths.values().sum();
            let total_distance = total_distance as f32;

            // The closeness centrality for the node is the inverse of the total distance
            // If a node is disconnected (total_distance is 0), its centrality is 0
            let centrality = if total_distance > 0.0 {
                (node_count - 1.0) / total_distance
            } else {
                0.0
            };

            let node_data = self.node_weight_mut(node_ind).unwrap();
            node_data.clostness_centrality = centrality;
        }
    }

    fn local_clustering_coefficient(&mut self) {
        for node in self.node_indices() {
            // Get all the successors and predecessors of the node
            let successors: Vec<NodeIndex> =
                self.neighbors_directed(node, Direction::Outgoing).collect();
            let predecessors: Vec<NodeIndex> =
                self.neighbors_directed(node, Direction::Incoming).collect();

            // Count the number of edges between successors and predecessors
            let mut edges_between_neighbors = 0;
            for &successor in &successors {
                for &predecessor in &predecessors {
                    if self.contains_edge(predecessor, successor) {
                        edges_between_neighbors += 1;
                    }
                }
            }

            // The number of possible edges between successors and predecessors
            let possible_edges = successors.len() * predecessors.len();

            // Calculate the local clustering coefficient for the node
            let coefficient = if possible_edges > 0 {
                edges_between_neighbors as f32 / possible_edges as f32
            } else {
                0.0 // If there are no successors or predecessors, the coefficient is 0.
            };

            let node_data = self.node_weight_mut(node).unwrap();
            // Store the coefficient for the node
            node_data.local_clustering_coefficient = coefficient;
        }
    }

    fn is_cyclic_directed(&self) -> bool {
        use petgraph::algo::is_cyclic_directed;
        is_cyclic_directed(self)
    }

    fn to_cyjson(&self) -> String {
        let mut nlgraph_json = serde_json::to_value(self).unwrap();
        let mut cy = serde_json::json!({
            "desinty": self.density(),
            "data": [],
            "directed": true,
            "multigraph": true,
            "elements": {
            "nodes": [],
            "edges": []
            }
        });

        let d = cy["elements"]["nodes"].as_array_mut().unwrap();
        if let serde_json::Value::Array(nl_nodes) = nlgraph_json["nodes"].take() {
            for node_props in nl_nodes {
                let node = serde_json::json!({ "data": node_props });
                d.push(node);
            }
        }

        let e = cy["elements"]["edges"].as_array_mut().unwrap();

        if let serde_json::Value::Array(nl_edges) = nlgraph_json["edges"].take() {
            for edge_props in nl_edges {
                // remove first and second element of edge_props
                let data = edge_props.as_array();

                let edge_data = data.iter().last().unwrap().iter().last().unwrap();
                let edge = serde_json::json!({ "data": edge_data});
                e.push(edge);
            }
        }

        serde_json::to_string_pretty(&cy).unwrap()
    }

    fn vis(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    const DATA: &str = r#"
     {
        "data": [],
        "directed": true,
        "multigraph": true,
        "elements": {
            "nodes": [
                {
                    "data": {
                        "chrom": "chr1",
                        "ref_start": 154220171,
                        "ref_end": 154261697,
                        "strand": "+",
                        "is_head": true,
                        "id": "chr1_154220171_154261697_H+",
                        "value": "chr1_154220171_154261697_H+",
                        "name": "chr1_154220171_154261697_H+"
                    }
                },
                {
                    "data": {
                        "chrom": "chr2",
                        "ref_start": 80617598,
                        "ref_end": 80666408,
                        "strand": "-",
                        "is_head": false,
                        "id": "chr2_80617598_80666408_T-",
                        "value": "chr2_80617598_80666408_T-",
                        "name": "chr2_80617598_80666408_T-"
                    }
                }
            ],
            "edges": [
                {
                    "data": {
                       "label": "TRA_(False, MicroHomology(G))_1",
                        "weight": 1,
                        "read_ids": ["m64135_201204_204719/97059215/ccs"],
                        "source": "chr1_154220171_154261697_H+",
                        "target": "chr2_80617598_80666408_T-",
                        "key": 0
                    }
                }
            ]
        }
     }"#;

    fn load_nlgraph() -> NLGraph {
        use crate::graph::load::load_cygraph_from_json;
        let mut nlgraph = load_cygraph_from_json(serde_json::from_str(DATA).unwrap()).unwrap();

        nlgraph.node_degree();
        nlgraph
    }

    #[test]
    fn test_node_degree() {
        let _nlgraph = load_nlgraph();
    }

    #[test]
    fn test_is_weakly_connected() {
        let nlgraph = load_nlgraph();
        assert!(nlgraph.is_weakly_connected());
    }

    #[test]
    fn test_walk() {
        let nlgraph = load_nlgraph();
        nlgraph.walk();
    }

    #[test]
    fn test_desinty() {
        let nlgraph = load_nlgraph();
        assert_eq!(nlgraph.density(), 1.0);
    }

    #[test]
    fn test_closeness_centrality() {
        let mut nlgraph = load_nlgraph();
        nlgraph.closeness_centrality();
    }

    #[test]
    fn test_degree_centrality() {
        let mut nlgraph = load_nlgraph();
        nlgraph.degree_centrality();
    }

    #[test]
    fn test_local_clustering_coefficient() {
        let mut nlgraph = load_nlgraph();
        nlgraph.local_clustering_coefficient();
    }

    #[test]
    fn test_to_json() {
        let nlgraph = load_nlgraph();

        let s = serde_json::to_string_pretty(&nlgraph);
        let sg: NLGraph = serde_json::from_str(s.as_ref().unwrap()).unwrap();
        println!("{:?}", sg);
    }

    #[test]
    fn test_cyjson() {
        let nlgraph = load_nlgraph;
        let _cyjson = nlgraph().to_cyjson();
        println!("{}", _cyjson);
    }
}
