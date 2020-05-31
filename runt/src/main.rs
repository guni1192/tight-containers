mod cli;
mod config;
mod container;
mod specutil;
mod subcommand;

use anyhow::Result;
use subcommand::{create::CreateCommand, spec::SpecCommand, SubCommand, SubCommandImpl};

fn main() -> Result<()> {
    let mut app = cli::app_config();
    let app_matches = &app.clone().get_matches();

    let subcommand_: SubCommand = match app_matches.subcommand() {
        ("create", Some(matches)) => SubCommand::Create(CreateCommand::new(matches)?),
        ("spec", Some(matches)) => SubCommand::Spec(SpecCommand::new(matches)?),
        _ => {
            app.print_help()?;
            std::process::exit(1);
        }
    };

    match subcommand_ {
        SubCommand::Create(command) => command.run()?,
        SubCommand::Spec(command) => command.run()?,
    }

    Ok(())
}
