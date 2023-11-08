use clap::{Command, CommandFactory, Parser, Subcommand, ValueHint};
use env_logger::Builder;
use human_panic::setup_panic;
use log::info;
use log::LevelFilter;
use std::path::PathBuf;

use clap_complete::{generate, Generator, Shell};
use std::io;

mod extract;
mod fa2fq;
mod fq2fa;
mod index;
mod rsoft;

mod graph;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    // If provided, outputs the completion file for given shell
    #[arg(long = "generate", value_enum)]
    generator: Option<Shell>,

    #[command(subcommand)]
    command: Option<Commands>,

    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Extract reads from a BAM file
    #[command(visible_alias = "e")]
    Extract {
        /// Read IDs
        #[arg(value_hint = ValueHint::FilePath)]
        readids: String,
        /// Bam input file
        #[arg(value_hint = ValueHint::FilePath)]
        input: PathBuf,

        /// Is the output file a BAM file
        #[arg(short = 'b', default_value = "false")]
        isbam: bool,
    },

    /// Index a BAM file
    Index {
        /// Bam input file
        #[arg(value_hint = ValueHint::FilePath)]
        input: PathBuf,
    },

    /// Convert a FASTA file to FASTQ
    Fa2fq {
        /// fasta input file
        #[arg(value_hint = ValueHint::FilePath)]
        input: PathBuf,
    },

    /// Convert a FASTQ file to FASTA
    Fq2fa {
        /// fastq input file
        #[arg(value_hint = ValueHint::FilePath)]
        input: PathBuf,
    },

    /// Create softlinks to files with suffix recursively
    Rsoft {
        /// The directory to search
        #[arg(value_hint = ValueHint::FilePath)]
        source: PathBuf,

        /// The directory to create the softlinks. default is current directory
        #[arg(short = 't', value_hint = ValueHint::FilePath)]
        target: Option<PathBuf>,

        /// The suffix of the files to link. default is all files
        #[arg(short = 's', value_delimiter = ' ', num_args=1..)]
        suffix: Option<Vec<String>>,

        /// Overwrite existing files
        #[arg(short = 'o', default_value = "false")]
        overwrite: bool,
    },

    /// Graph Analysis
    Graph(graph::GraphArgs),
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() {
    setup_panic!();

    let cli = Cli::parse();

    let mut log_builder = Builder::from_default_env();

    match cli.verbose.log_level() {
        Some(level) => {
            info!("Verbose mode is on with level {}!", level);
            log_builder.filter(None, level.to_level_filter());
        }
        None => {
            info!("Verbose mode is off!");
            log_builder.filter(None, LevelFilter::Off);
        }
    }
    log_builder.init();

    if let Some(generator) = cli.generator {
        let mut cmd = Cli::command();
        info!("Generating completion file for {generator:?}...");
        print_completions(generator, &mut cmd);
        return;
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Extract {
            readids,
            input,
            isbam,
        }) => {
            info!("'extract'  {readids:?} from {input:?} ");
            extract::extract(readids, input, *isbam).unwrap();
        }

        Some(Commands::Index { input }) => {
            info!("'index'  {input:?} ");
            index::index_bam(input).unwrap();
        }

        Some(Commands::Fa2fq { input }) => {
            info!("'fa2fq'  {input:?} ");
            fa2fq::fa2fq(input).unwrap();
        }

        Some(Commands::Fq2fa { input }) => {
            info!("'fq2fa'  {input:?} ");
            fq2fa::fq2fa(input).unwrap();
        }

        Some(Commands::Rsoft {
            source,
            target,
            suffix,
            overwrite,
        }) => {
            info!("'rsoft'  {target:?} {suffix:?} ");
            rsoft::rsoft(source, target.as_ref(), suffix.clone(), *overwrite).unwrap();
        }

        Some(Commands::Graph(args)) => {
            info!("'graph'  {args:?} ");
        }

        // If no subcommand was used, it's a normal top level command
        None => info!("No subcommand was used"),
    }
}
