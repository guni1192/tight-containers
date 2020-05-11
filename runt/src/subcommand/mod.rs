pub mod spec;

use anyhow::Result;
use clap::ArgMatches;

pub trait SubCommand: Sized {
    fn new(matches: &ArgMatches) -> Result<Self>;
    fn run(&self) -> Result<()>;
}
