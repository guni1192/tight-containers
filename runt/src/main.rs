mod cli;
mod config;
mod container;
mod error;
mod subcommand;

use subcommand::{spec::SpecCommand, SubCommand};

fn main() -> Result<(), error::Error> {
    let app_maches = cli::app_config().get_matches();

    let subcommand = match app_maches.subcommand() {
        ("spec", Some(matches)) => SpecCommand::new(matches),
        _ => Err(error::invalid_input("invalid subcommand")),
    }?;

    subcommand.run()?;

    Ok(())
}
