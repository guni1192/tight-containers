pub mod create;
pub mod spec;
pub mod state;

use anyhow::Result;
use clap::ArgMatches;

use crate::subcommand::{create::CreateCommand, spec::SpecCommand, state::StateCommand};

pub enum SubCommand {
    Create(CreateCommand),
    Spec(SpecCommand),
    State(StateCommand),
}

pub trait SubCommandImpl: Sized {
    fn new(matches: &ArgMatches) -> Result<Self>;
    fn run(&self) -> Result<()>;
}
