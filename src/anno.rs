use anyhow::Result;
use clap::{arg, Args, ValueHint};
use log::{error, info, warn};
use std::path::{Path, PathBuf};
use std::str::FromStr;

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

#[derive(Debug, Default)]
pub struct RecordInfo {
    // NONCANONICAL;BOUNDARY=NEITHER;SVTYPE=TRA;SR=1;OSR
    // =1;CHR2=chr6;SVEND=57111495;DP1=10;DP2=710;PSI=0.00277;SVLEN=0;GENE1=RNF223;GENE2=ZN
    // F451;MEGAEXON1=1;MEGAEXON2=2;STRAND1=-;STRAND2=+;MODE1=SM;MODE2=SM;HOMSEQ=GCTGAG;INS
    // SEQ=.;TRANSCRIPT_ID=1x1;GENE_ID=1;SR_ID=m64135_220622_211525/178979354/ccs|another;SVMETHOD=
    // ScanNLS
    data: String,

    svtype: String,
    sr: u64,
    osr: u64,
    transcript_id: String,
    gene_id: String,
    sr_id: Vec<String>,
}

impl FromStr for RecordInfo {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let info = s.split(';').collect::<Vec<&str>>();
        let svtype = info[2].split('=').collect::<Vec<&str>>()[1].to_string();
        let sr = info[3].split('=').collect::<Vec<&str>>()[1]
            .parse::<u64>()
            .unwrap();
        let osr = info[4].split('=').collect::<Vec<&str>>()[1]
            .parse::<u64>()
            .unwrap();

        let transcript_id = info[21].split('=').collect::<Vec<&str>>()[1].to_string();
        let gene_id = info[22].split('=').collect::<Vec<&str>>()[1].to_string();
        let sr_id = info[23].split('=').collect::<Vec<&str>>()[1]
            .split('|')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        Ok(RecordInfo {
            data: s.to_string(),
            svtype,
            sr,
            osr,
            transcript_id,
            gene_id,
            sr_id,
        })
    }
}

#[derive(Debug, Default)]
pub struct VcfRecord {
    chrom: String,
    pos: u64,
    id: String,
    ref_allele: String,
    alt_allele: String,
    qual: String,
    filter: String,
    info: RecordInfo,
    format: String,
    sample: String,
}

impl FromStr for VcfRecord {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split('\t').collect();
        let chrom = fields[0].to_string();
        let pos = fields[1].parse::<u64>().unwrap();
        let id = fields[2].to_string();
        let ref_allele = fields[3].to_string();
        let alt_allele = fields[4].to_string();
        let qual = fields[5].to_string();
        let filter = fields[6].to_string();
        let info = fields[7].to_string();
        let format = fields[8].to_string();
        let sample = fields[9].to_string();
        Ok(VcfRecord {
            chrom,
            pos,
            id,
            ref_allele,
            alt_allele,
            qual,
            filter,
            info: RecordInfo::from_str(&info).unwrap(),
            format,
            sample,
        })
    }
}

pub fn read_vcf<P: AsRef<Path>>(vcf_path: P) -> Result<Vec<VcfRecord>> {
    let vcf = read_lines(vcf_path)?;
    Ok(vcf
        .filter_map(|line| {
            if let Ok(row) = line {
                if !row.starts_with('#') {
                    let record = VcfRecord::from_str(&row).unwrap();
                    Some(record)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<VcfRecord>>())
}

pub fn read_bam<P: AsRef<Path>>(bam_path: P) {
    todo!()
}

pub fn read_fasta<P: AsRef<Path>>(fasta_path: P) {
    todo!()
}
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
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

    #[test]
    fn test_read_vcf() {
        let vcf_path = "tests/data/lncap_test.vcf";
        let records = read_vcf(vcf_path).unwrap();
        println!("{:?}", records[0].info);
    }
}
