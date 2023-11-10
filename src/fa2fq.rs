use anyhow::Result;
use noodles_fasta as fasta;
use noodles_fastq as fastq;
use std::path::Path;

pub fn fa2fq<P: AsRef<Path>>(input: P) -> Result<()> {
    let mut reader = fasta::reader::Builder.build_from_path(input)?;
    let mut writer = fastq::Writer::new(std::io::stdout());

    for result in reader.records() {
        let record = result?;
        let name = record.name().to_string();
        let sequence = record.sequence().as_ref().to_vec();
        let qualities = vec![b'@'; sequence.len()];
        let fastq_record = fastq::Record::new(
            fastq::record::Definition::new(name, ""),
            sequence,
            qualities,
        );
        writer.write_record(&fastq_record)?;
    }

    Ok(())
}
