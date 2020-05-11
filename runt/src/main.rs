mod cli;
mod config;
mod container;
mod subcommand;

use anyhow::Result;
use subcommand::{spec::SpecCommand, SubCommand};

fn main() -> Result<()> {
    let mut app = cli::app_config();
    let app_matches = &app.clone().get_matches();

    let subcommand = match app_matches.subcommand() {
        ("spec", Some(matches)) => SpecCommand::new(matches),
        _ => {
            app.print_help()?;
            std::process::exit(1);
        }
    }?;

    subcommand.run()?;

    Ok(())
}
