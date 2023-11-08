use anyhow::Result;
use log::info;
use petgraph::Graph;
use serde_json::Value;
use std::path::Path;

use crate::graph::data::{EdgeData, NodeData};

pub fn load_cygraph<P: AsRef<Path>>(input: P) -> Result<()> {
    info!("Loading graph from {:?}", input.as_ref());
    let reader = std::io::BufReader::new(std::fs::File::open(input.as_ref())?);
    let data = serde_json::from_reader(reader)?;

    let mut graph = Graph::<NodeData, EdgeData>::new();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loadcygraph() {
        let data = r#"
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

        let v: Value = serde_json::from_str(data).unwrap();
        serde_json::to_writer_pretty(std::io::stdout(), &v).unwrap();
    }
}
