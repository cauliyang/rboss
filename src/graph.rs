use clap::Args;
use clap::ValueHint;
use std::path::PathBuf;

mod load;

#[derive(Args, Debug)]
struct GraphArgs {
    /// Graph input file
    #[arg(value_hint = ValueHint::FilePath)]
    input: PathBuf,
}
