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
    pub id: String,
    pub label: String,
    pub weight: u64,
    pub read_ids: Vec<String>,
}
