use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;

use crate::config::SPEC_FILE;
use crate::container::specs::Spec;

pub fn load(bundle: &PathBuf) -> Result<Spec> {
    let config_path = bundle.join(SPEC_FILE);
    let config_file = File::open(&config_path)?;

    let spec: Spec = serde_json::from_reader(&config_file)?;

    Ok(spec)
}

pub fn write(bundle: &PathBuf, spec: &Spec) -> Result<()> {
    let config_path = bundle.join(SPEC_FILE);
    let mut config_file = File::create(&config_path)?;

    serde_json::to_writer(&mut config_file, &spec)?;
    Ok(())
}
