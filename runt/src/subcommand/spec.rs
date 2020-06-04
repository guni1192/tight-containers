use std::path::PathBuf;

use anyhow::Result;
use clap::ArgMatches;

use crate::container::specs::Spec;
use crate::specutil;
use crate::subcommand::SubCommandImpl;

pub struct SpecCommand {
    pub bundle: PathBuf,
}

impl SubCommandImpl for SpecCommand {
    fn new(matches: &ArgMatches) -> Result<Self> {
        let bundle = PathBuf::from(matches.value_of("bundle").unwrap_or(".")).canonicalize()?;
        Ok(SpecCommand { bundle })
    }
    fn run(&self) -> Result<()> {
        specutil::write(&self.bundle, &Spec::default())?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use tempfile::tempdir;

    use crate::cli::app_config;
    use crate::config::SPEC_FILE;
    use crate::container::testutil;

    #[test]
    fn bundle_should_be_current_dir() {
        let args = vec!["runt", "spec"];

        let app_matches = app_config()
            .get_matches_from_safe(&args)
            .unwrap_or_else(|e| panic!("An error occurs: {}", e));

        let subcommand = match app_matches.subcommand() {
            ("spec", Some(matches)) => Some(SpecCommand::new(matches)),
            _ => None,
        }
        .unwrap()
        .unwrap();

        assert_eq!(
            subcommand.bundle,
            PathBuf::from(".").canonicalize().unwrap()
        )
    }

    #[test]
    fn bundle_should_be_specify_dir() {
        let bundle = testutil::init_bundle_dir().unwrap();
        let args = vec!["runt", "spec", "-b", bundle.to_str().unwrap()];

        let app_matches = app_config()
            .get_matches_from_safe(&args)
            .unwrap_or_else(|e| panic!("An error occurs: {}", e));

        let subcommand = match app_matches.subcommand() {
            ("spec", Some(matches)) => Some(SpecCommand::new(matches)),
            _ => None,
        }
        .unwrap()
        .unwrap();

        assert_eq!(subcommand.bundle, bundle);

        testutil::cleanup(&[&bundle]).unwrap();
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
            ("spec", Some(matches)) => Some(SpecCommand::new(matches)),
            _ => None,
        }
        .unwrap()
        .unwrap();
        subcommand.run().unwrap();

        assert!(bundle.join(SPEC_FILE).exists())
    }
}
