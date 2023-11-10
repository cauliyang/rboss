use anyhow::Result;
use log::info;
use serde_json::Value;
use std::path::Path;

use std::collections::HashMap;

use crate::graph::data::{EdgeData, NLGraph, NodeData};

pub fn load_cygraph_from_file<P: AsRef<Path>>(file: P) -> Result<NLGraph> {
    let reader = std::io::BufReader::new(std::fs::File::open(file.as_ref())?);
    let data: Value = serde_json::from_reader(reader)?;
    load_cygraph_from_json(data)
}

pub fn load_cygraph_from_json(data: Value) -> Result<NLGraph> {
    let nodes = data.get("elements").unwrap().get("nodes").unwrap();
    let edges = data.get("elements").unwrap().get("edges").unwrap();

    let node_number = nodes.as_array().unwrap().len();
    let edge_number = edges.as_array().unwrap().len();

    let mut graph = NLGraph::with_capacity(node_number, edge_number);

    let mut id2index = HashMap::new();

    for node in nodes.as_array().unwrap() {
        let node_data = NodeData::from_json(node);
        let id = node_data.id.clone();
        let index = graph.add_node(node_data);
        id2index.insert(id, index);
    }

    for edge in edges.as_array().unwrap() {
        let edge_data = EdgeData::from_json(edge);
        let source = id2index.get(&edge_data.source).unwrap();
        let target = id2index.get(&edge_data.target).unwrap();
        let _index = graph.add_edge(*source, *target, edge_data);
    }

    info!("Added {} nodes", graph.node_count());
    info!("Added {} edges", graph.edge_count());

    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn test_loadcygraph() {
        load_cygraph_from_json(serde_json::from_str(DATA).unwrap()).unwrap();
    }
}
