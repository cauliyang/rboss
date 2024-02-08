use std::io;
use std::path::Path;

use noodles_core::Position;

use noodles_bam::{self as bam, bai};
use noodles_csi::binning_index::{index::reference_sequence::bin::Chunk, Indexer};
use noodles_sam::{self as sam};
use sam::alignment::RecordBuf;

fn is_coordinate_sorted(header: &sam::Header) -> bool {
    use sam::header::record::value::map::header::{sort_order, tag};

    header
        .header()
        .and_then(|hdr| hdr.other_fields().get(&tag::SORT_ORDER))
        .map(|sort_order| sort_order == sort_order::COORDINATE)
        .unwrap_or_default()
}

fn alignment_context(
    record: &sam::alignment::RecordBuf,
) -> io::Result<(Option<usize>, Option<Position>, Option<Position>)> {
    Ok((
        record.reference_sequence_id(),
        record.alignment_start(),
        record.alignment_end(),
    ))
}

pub fn index_bam<P: AsRef<Path>, W: io::Write>(file: P, index_file: Option<W>) -> io::Result<()> {
    let mut reader = bam::io::reader::Builder.build_from_path(file.as_ref())?;
    let header = reader.read_header()?;

    if !is_coordinate_sorted(&header) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "the input BAM must be coordinate-sorted to be indexed",
        ));
    }

    let mut record = RecordBuf::default();

    let mut builder = Indexer::default();
    let mut start_position = reader.virtual_position();

    while reader.read_record_buf(&header, &mut record)? != 0 {
        let end_position = reader.virtual_position();
        let chunk = Chunk::new(start_position, end_position);

        let alignment_context = match alignment_context(&record)? {
            (Some(id), Some(start), Some(end)) => {
                let is_mapped = !record.flags().is_unmapped();
                Some((id, start, end, is_mapped))
            }
            _ => None,
        };

        builder.add_record(alignment_context, chunk)?;

        start_position = end_position;
    }

    let index = builder.build(header.reference_sequences().len());

    if let Some(index_file) = index_file {
        let mut writer = bai::Writer::new(index_file);
        writer.write_index(&index)?;
    } else {
        let stdout = io::stdout().lock();
        let mut writer = bai::Writer::new(stdout);
        writer.write_index(&index)?;
    }

    Ok(())
}
