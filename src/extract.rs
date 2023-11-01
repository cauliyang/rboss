use anyhow::Result;
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use noodles_bam as bam;
use noodles_sam::{self as sam, record::ReadName};

use std::path::PathBuf;

fn writer(file: Option<&PathBuf>, is_bam: bool) -> Result<Box<dyn sam::AlignmentWriter>> {
    let sink: Box<dyn io::Write> = if let Some(file) = file {
        Box::new(File::create(file)?)
    } else {
        Box::new(io::stdout().lock())
    };

    let writer: Box<dyn sam::AlignmentWriter> = if is_bam {
        Box::new(bam::Writer::new(sink))
    } else {
        Box::new(sam::Writer::new(sink))
    };

    Ok(writer)
}

fn read_read_names<P>(src: P) -> Result<HashSet<ReadName>>
where
    P: AsRef<Path>,
{
    let reader = File::open(src).map(BufReader::new)?;
    let mut read_names = HashSet::new();

    for result in reader.lines() {
        let read_name = result.and_then(|s| {
            ReadName::try_from(s.into_bytes())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })?;

        read_names.insert(read_name);
    }

    Ok(read_names)
}

pub fn extract<P>(read_ids: &str, bam_file: P, is_bam: bool) -> Result<()>
where
    P: AsRef<std::path::Path>,
{
    let read_names = read_read_names(read_ids)?;

    let mut reader = bam::reader::Builder.build_from_path(bam_file)?;
    let header = reader.read_header()?;

    let mut writer = writer(None, is_bam)?;

    writer.write_alignment_header(&header)?;

    for result in reader.records(&header) {
        let record = result?;
        if let Some(read_name) = record.read_name() {
            if read_names.contains(read_name) {
                writer.write_alignment_record(&header, &record)?;
            }
        }
    }
    Ok(())
}
