use std::io;
use std::path::Path;

use noodles_bam::{self as bam, bai};
use noodles_csi::binning_index::{index::reference_sequence::bin::Chunk, Indexer};
use noodles_sam::{self as sam, alignment::Record};

fn is_coordinate_sorted(header: &sam::Header) -> bool {
    use sam::header::record::value::map::header::SortOrder;

    if let Some(hdr) = header.header() {
        if let Some(sort_order) = hdr.sort_order() {
            return sort_order == SortOrder::Coordinate;
        }
    }

    false
}

pub fn index_bam<P: AsRef<Path>>(file: P) -> io::Result<()> {
    let mut reader = bam::reader::Builder.build_from_path(file.as_ref())?;
    let header = reader.read_header()?;

    if !is_coordinate_sorted(&header) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "the input BAM must be coordinate-sorted to be indexed",
        ));
    }

    let mut record = Record::default();

    let mut builder = Indexer::default();
    let mut start_position = reader.virtual_position();

    while reader.read_record(&header, &mut record)? != 0 {
        let end_position = reader.virtual_position();
        let chunk = Chunk::new(start_position, end_position);

        let alignment_context = match (
            record.reference_sequence_id(),
            record.alignment_start(),
            record.alignment_end(),
        ) {
            (Some(id), Some(start), Some(end)) => {
                Some((id, start, end, !record.flags().is_unmapped()))
            }
            _ => None,
        };

        builder.add_record(alignment_context, chunk)?;

        start_position = end_position;
    }

    let index = builder.build(header.reference_sequences().len());
    // let index_file = file.as_ref().with_extension("bai");

    // write to index file
    let stdout = io::stdout().lock();
    let mut writer = bai::Writer::new(stdout);
    writer.write_index(&index)?;

    Ok(())
}
