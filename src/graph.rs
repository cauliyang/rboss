use clap::Args;
use clap::ValueHint;
use std::path::PathBuf;

mod analysis;
mod data;
mod load;

#[derive(Args, Debug)]
pub struct GraphArgs {
    /// Graph input file
    #[arg(value_hint = ValueHint::FilePath)]
    input: PathBuf,
}
