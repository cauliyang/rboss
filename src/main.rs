use clap::{Parser, Subcommand};
use env_logger::Builder;
use human_panic::setup_panic;
use log::max_level;
use log::{debug, error, info, trace, warn, LevelFilter};
use std::path::PathBuf;

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
        #[arg(short, long)]
        readids: Option<String>,
        /// Bam input file
        #[arg(short, long)]
        input: PathBuf,
    },
}

fn main() {
    setup_panic!();

    let cli = Cli::parse();

    let mut log_builder = Builder::from_default_env();

    match cli.verbose.log_level() {
        Some(level) => {
            println!("Verbose mode is on with level {}!", level);
            log_builder.filter(None, level.to_level_filter());
        }
        None => {
            println!("Verbose mode is off!");
            log_builder.filter(None, LevelFilter::Off);
        }
    }
    log_builder.init();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Extract { readids, input }) => {
            println!("'extract'  {readids:?} from {input:?} ")
        }

        // If no subcommand was used, it's a normal top level command
        None => println!("No subcommand was used"),
    }
}
