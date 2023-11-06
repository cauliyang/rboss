use anyhow::Result;
use noodles_fasta as fasta;
use noodles_fastq as fastq;
use std::path::Path;
use std::{
    fs::File,
    io::{self, BufReader},
};

pub fn fq2fa<P: AsRef<Path>>(input: P) -> Result<()> {
    let mut readerr = File::open(input)
        .map(BufReader::new)
        .map(fastq::Reader::new)?;

    let mut writer = fasta::Writer::new(io::stdout());

    for result in readerr.records() {
        let record = result?;
        let name = String::from_utf8(record.name().to_vec())?;
        let sequence = fasta::record::Sequence::from(record.sequence().to_vec());
        let fasta_record = fasta::Record::new(fasta::record::Definition::new(name, None), sequence);
        writer.write_record(&fasta_record)?;
    }

    Ok(())
}
