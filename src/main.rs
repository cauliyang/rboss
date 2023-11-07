use clap::{Parser, Subcommand};
use env_logger::Builder;
use human_panic::setup_panic;
use log::info;
use log::LevelFilter;
use std::path::PathBuf;

mod extract;
mod fa2fq;
mod fq2fa;
mod index;
mod rsoft;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract reads from a BAM file
    Extract {
        /// Read IDs
        readids: String,
        /// Bam input file
        input: PathBuf,

        /// Is the output file a BAM file
        #[arg(short = 'b', default_value = "false")]
        isbam: bool,
    },

    /// Index a BAM file
    Index {
        /// Bam input file
        input: PathBuf,
    },

    /// Convert a FASTA file to FASTQ
    Fa2fq {
        /// fasta input file
        input: PathBuf,
    },

    /// Convert a FASTQ file to FASTA
    Fq2fa {
        /// fastq input file
        input: PathBuf,
    },

    /// Create softlinks to files with same suffix in one directory recursively
    Rsoft {
        /// The directory to search
        target: PathBuf,
        /// The suffix of the files to link default is all files
        #[arg(short = 's')]
        suffix: Option<String>,
    },
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

        Some(Commands::Rsoft { target, suffix }) => {
            info!("'rsoft'  {target:?} {suffix:?} ");
            rsoft::rsoft(target, suffix.clone()).unwrap();
        }

        // If no subcommand was used, it's a normal top level command
        None => info!("No subcommand was used"),
    }
}
