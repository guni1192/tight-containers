use std::fs::File;
use std::path::PathBuf;

use clap::ArgMatches;

use crate::config::SPEC_FILE;
use crate::container::specs::Spec;
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
        let file = File::create(self.bundle.join(SPEC_FILE))?;
        serde_json::to_writer(file, &Spec::default())?;

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::cli::app_config;
    use crate::error::invalid_input;
    use tempfile::tempdir;

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

    #[test]
    fn should_exsist_spec_file() {
        let tempdir = tempdir().unwrap();
        let bundle = tempdir.path();

        let args = vec!["runt", "spec", "-b", bundle.to_str().unwrap()];

        let app_matches = app_config()
            .get_matches_from_safe(&args)
            .unwrap_or_else(|e| panic!("An error occurs: {}", e));

        let subcommand = match app_matches.subcommand() {
            ("spec", Some(matches)) => SpecCommand::new(matches),
            _ => Err(invalid_input("invalid subcommand")),
        }
        .unwrap();
        subcommand.run().unwrap();

        assert!(bundle.join(SPEC_FILE).exists())
    }
}
