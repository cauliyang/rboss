use anyhow::Result;
use log::info;
use std::path::Path;

#[allow(dead_code)]
fn load_cygraph<P: AsRef<Path>>(input: P) -> Result<()> {
    info!("Loading graph from {:?}", input.as_ref());

    Ok(())
}
