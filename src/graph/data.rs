use serde_json::Value;
use std::str::FromStr;

#[derive(Debug)]
pub enum Strand {
    Positive,
    Negative,
}

impl Strand {
    pub fn new(s: &str) -> Self {
        match s {
            "+" => Strand::Positive,
            "-" => Strand::Negative,
            _ => panic!("Invalid strand"),
        }
    }

    pub fn is_reverse(&self) -> bool {
        match self {
            Strand::Positive => false,
            Strand::Negative => true,
        }
    }
}

impl FromStr for Strand {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Strand::Positive),
            "-" => Ok(Strand::Negative),
            _ => panic!("Invalid strand"),
        }
    }
}

#[derive(Debug)]
pub struct NodeData {
    pub id: String,
    pub label: String,
    pub chrom: String,
    pub ref_start: u64,
    pub ref_end: u64,
    pub strand: Strand,
    pub is_head: bool,
}

#[derive(Debug)]
pub struct EdgeData {
    pub label: String,
    pub weight: u64,
    pub read_ids: Vec<String>,
    pub source: String,
    pub target: String,
}

impl NodeData {
    pub fn from_json(node_data: &Value) -> Self {
        // node: Object {"data": Object {"chrom": String("chr1"),
        // "id": String("chr1_154220171_154261697_H+"
        // ), "is_head": Bool(true), "name": String("chr1_154220171_154261697_H+"),
        // "ref_end": Number(154261697),
        // "ref_start": Number(154220171),
        // "strand": String("+"),
        // "value": String("chr1_154220171_154261697_H+")}}

        let node = node_data.get("data").unwrap();
        let id = node.get("id").unwrap().as_str().unwrap().to_string();
        let label = node.get("name").unwrap().as_str().unwrap().to_string();
        let chrom = node.get("chrom").unwrap().as_str().unwrap().to_string();
        let ref_start = node.get("ref_start").unwrap().as_u64().unwrap();
        let ref_end = node.get("ref_end").unwrap().as_u64().unwrap();
        let strand = Strand::new(node.get("strand").unwrap().as_str().unwrap());
        let is_head = node.get("is_head").unwrap().as_bool().unwrap();
        Self {
            id,
            label,
            chrom,
            ref_start,
            ref_end,
            strand,
            is_head,
        }
    }
}

impl EdgeData {
    pub fn from_json(edge_data: &Value) -> Self {
        //         Object {"data": Object {"key": Number(0),
        //         "label": String("TRA_(False, MicroHomology(G))_1"),
        //         "read_ids": Array [String("m64135_201204_204719/97059215/ccs")],
        //         "source": String("chr1_154220171_154261697_H+"),
        //         "target": String("chr2_80617598_80666408_T-"),
        //         "weight": Number(1)}}

        let edge = edge_data.get("data").unwrap();
        let label = edge.get("label").unwrap().as_str().unwrap().to_string();
        let weight = edge.get("weight").unwrap().as_u64().unwrap();
        let read_ids = edge
            .get("read_ids")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();

        let source = edge.get("source").unwrap().as_str().unwrap().to_string();
        let target = edge.get("target").unwrap().as_str().unwrap().to_string();

        Self {
            label,
            weight,
            read_ids,
            source,
            target,
        }
    }
}
