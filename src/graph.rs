use anyhow::Result;
use clap::Args;
use clap::ValueHint;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

mod analysis;
mod data;
mod load;

use analysis::GraphAnalysis;
use log::warn;

#[derive(Args, Debug)]
pub struct GraphArgs {
    /// Graph input file
    #[arg(value_hint = ValueHint::FilePath)]
    input: PathBuf,
}

pub fn analyze_nlgraph(args: &GraphArgs) -> Result<()> {
    let mut nlgraph = load::load_cygraph_from_file(&args.input)?;

    if !nlgraph.is_weakly_connected() {
        warn!("Graph is weakly connected");
        return Ok(());
    }

    if nlgraph.is_cyclic_directed() {
        warn!("Graph is cyclic");
        return Ok(());
    }

    nlgraph.node_degree();
    nlgraph.degree_centrality();
    nlgraph.closeness_centrality();
    nlgraph.local_clustering_coefficient();

    let stdout = std::io::stdout().lock();
    let mut handle = BufWriter::new(stdout); // optional: wrap that handle in a buffer
    writeln!(handle, "{}", nlgraph.to_cyjson())?;

    Ok(())
}
