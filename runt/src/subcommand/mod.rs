pub mod create;
pub mod spec;

use anyhow::Result;
use clap::ArgMatches;

use crate::subcommand::{create::CreateCommand, spec::SpecCommand};

pub enum SubCommand {
    Create(CreateCommand),
    Spec(SpecCommand),
}

pub trait SubCommandImpl: Sized {
    fn new(matches: &ArgMatches) -> Result<Self>;
    fn run(&self) -> Result<()>;
}
