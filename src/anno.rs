use anyhow::Result;
use clap::{arg, Args, ValueHint};
use log::{error, info, warn};
use std::path::{Path, PathBuf};

use polars::df;
use polars::prelude::*;
use std::io::{self, BufRead}; // Add this line

use bio::alignment::pairwise::Scoring;
use bio::alignment::poa::*;
use std::fs::File;

#[derive(Args, Debug)]
pub struct AnnoArgs {
    #[arg(value_hint = ValueHint::AnyPath)]
    pub vcf: PathBuf,
    // #[arg(value_hint = ValueHint::AnyPath)]
    // fasta: PathBuf,
    // #[arg(value_hint = ValueHint::AnyPath)]
    // bam: PathBuf,
}

pub fn read_vcf<P: AsRef<Path>>(vcf_path: P) -> Result<()> {
    let mut n = 0;

    let df = CsvReader::from_path(vcf_path.as_ref())?
        .with_separator(b'\t')
        .has_header(false)
        .with_comment_prefix(Some("#"))
        .finish()?;

    for (idx, row) in df.iter_chunks().enumerate() {
        if idx == 0 {
            for col in row.iter() {
                println!("{:?}", col);
            }
        }
    }

    // print every row

    // reader.records(&header).for_each(|r| match r {
    //     Ok(record) => {
    //         n += 1;
    //     }
    //     Err(e) => {
    //         error!("error: {:?}", e);
    //     }
    // });

    Ok(())
}

pub fn read_bam<P: AsRef<Path>>(bam_path: P) {
    todo!()
}

pub fn read_fasta<P: AsRef<Path>>(fasta_path: P) {
    todo!()
}
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run_poa<P: AsRef<Path>>(file: P) -> Result<()> {
    if let Ok(lines) = read_lines(file) {
        let r = lines.collect::<Result<Vec<String>, _>>()?;
        let flattened: Vec<&str> = r.iter().map(|line| line.as_str()).collect();
        let consensus = poa(&flattened);
        println!("{}", consensus);
    }
    Ok(())
}

pub fn poa(seqs: &[&str]) -> String {
    let scoring = Scoring::new(-2, 0, |a: u8, b: u8| if a == b { 1i32 } else { -1i32 });

    let mut seq_iter = seqs.iter().map(|s| s.as_bytes());
    let mut aligner = Aligner::new(scoring, seq_iter.next().unwrap());

    for seq in seq_iter {
        aligner.global(seq).add_to_graph();
    }

    String::from_utf8(aligner.consensus()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poa() {
        let seqs = vec![
            "ATATTGTGTAAGGCACAATTAACA",
            "ATATTGCAAGGCACAATTCAACA",
            "ATATTGCAAGGCACACAACA",
            "ATGTGCAAGAGCACATAAC",
        ];

        let test_seq = "ATATTGCAAGGCACAATTCAACA";
        let consensus = poa(&seqs);
        assert_eq!(consensus, test_seq);
    }

    #[test]
    fn test_extend() {
        let seqs = vec![
            "ATATTGTGTAAGGCACAATTAACA",
            "CAATTAACATTTTTTTTTTTTTTTTT",
            "ATGTGCAAGAGCACATAAC",
        ];

        println!("{}", poa(&seqs));
    }
}
