use anyhow::Result;
use clap::Args;
use clap::ValueHint;
use log::error;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

mod analysis;
mod data;
mod load;
// mod vis;

use analysis::GraphAnalysis;
use log::info;
use log::warn;

use self::data::NLGraph;

#[derive(Args, Debug)]
pub struct GraphArgs {
    /// Graph input file
    #[arg(value_hint = ValueHint::AnyPath)]
    input: PathBuf,

    /// current threads number
    #[arg(short = 't', default_value = "2")]
    threads: usize,
}

pub fn analyze(args: &GraphArgs) -> Result<()> {
    if args.input.is_dir() {
        info!("Analyzing graphs in directory {}", args.input.display());
        rayon::ThreadPoolBuilder::new()
            .num_threads(args.threads)
            .build_global()
            .unwrap();

        let mut nlgraphs = load::load_cygraph_from_directory(&args.input)?;
        return analyze_nlgraphs(&mut nlgraphs);
    } else if args.input.is_file() {
        info!("Analyzing graph in file {}", args.input.display());
        let mut nlgraph = load::load_cygraph_from_file(&args.input)?;
        return analyze_nlgraph(&mut nlgraph);
    }
    error!("Input is not a file or directory");
    Ok(())
}

pub fn analyze_nlgraphs(nlgraphs: &mut [NLGraph]) -> Result<()> {
    rayon::scope(|s| {
        for nlgraph in nlgraphs.iter_mut() {
            s.spawn(move |_| {
                analyze_nlgraph(nlgraph).unwrap();
            });
        }
    });

    Ok(())
}

pub fn analyze_nlgraph(nlgraph: &mut NLGraph) -> Result<()> {
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
