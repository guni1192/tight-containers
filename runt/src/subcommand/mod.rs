pub mod spec;

use clap::ArgMatches;

use crate::error::Error;

pub trait SubCommand: Sized {
    fn new(matches: &ArgMatches) -> Result<Self, Error>;
    fn run(&self) -> Result<(), Error>;
}
