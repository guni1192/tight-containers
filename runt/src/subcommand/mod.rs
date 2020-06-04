pub mod create;
pub mod delete;
pub mod spec;
pub mod state;

use anyhow::Result;
use clap::ArgMatches;

use crate::subcommand::create::CreateCommand;
use crate::subcommand::delete::DeleteCommand;
use crate::subcommand::spec::SpecCommand;
use crate::subcommand::state::StateCommand;

pub enum SubCommand {
    Create(CreateCommand),
    Spec(SpecCommand),
    State(StateCommand),
    Delete(DeleteCommand),
}

pub trait SubCommandImpl: Sized {
    fn new(matches: &ArgMatches) -> Result<Self>;
    fn run(&self) -> Result<()>;
}
