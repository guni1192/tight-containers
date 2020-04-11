use std::path::PathBuf;

use clap::ArgMatches;

use crate::error::Error;
use crate::subcommand::SubCommand;

pub struct SpecCommand {
    pub bundle: PathBuf,
}

impl SubCommand for SpecCommand {
    fn new(matches: &ArgMatches) -> Result<Self, Error> {
        let bundle = PathBuf::from(matches.value_of("bundle").unwrap_or("."));
        Ok(SpecCommand { bundle })
    }
    fn run(&self) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::cli::app_config;
    use crate::error::invalid_input;

    #[test]
    fn bundle_should_be_current_dir() {
        let args = vec!["runt", "spec"];

        let app_matches = app_config()
            .get_matches_from_safe(&args)
            .unwrap_or_else(|e| panic!("An error occurs: {}", e));

        let subcommand = match app_matches.subcommand() {
            ("spec", Some(matches)) => SpecCommand::new(matches),
            _ => Err(invalid_input("invalid subcommand")),
        }
        .unwrap();

        assert_eq!(subcommand.bundle, PathBuf::from("."))
    }

    #[test]
    fn bundle_should_be_specify_dir() {
        let args = vec!["runt", "spec", "-b", "/tmp/bundle"];

        let app_matches = app_config()
            .get_matches_from_safe(&args)
            .unwrap_or_else(|e| panic!("An error occurs: {}", e));

        let subcommand = match app_matches.subcommand() {
            ("spec", Some(matches)) => SpecCommand::new(matches),
            _ => Err(invalid_input("invalid subcommand")),
        }
        .unwrap();

        assert_eq!(subcommand.bundle, PathBuf::from("/tmp/bundle"))
    }
}
