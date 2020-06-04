use anyhow::Result;
use clap::ArgMatches;

use crate::container::{Container, MetadataManager};
use crate::subcommand::SubCommandImpl;

pub struct DeleteCommand {
    pub container_id: String,
}

impl SubCommandImpl for DeleteCommand {
    fn new(matches: &ArgMatches) -> Result<Self> {
        let container_id = matches
            .value_of("container-id")
            .expect("container-id must be specify:");

        Ok(DeleteCommand {
            container_id: container_id.into(),
        })
    }

    fn run(&self) -> Result<()> {
        let container = Container::load(&self.container_id)?;
        container.delete()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    use uuid::Uuid;

    use crate::cli::app_config;
    use crate::container::{testutil, DEFAULT_META_ROOT};
    use crate::specutil;

    #[test]
    fn should_be_container_deleted() {
        let container_id = Uuid::new_v4().to_string();
        let bundle = testutil::init_bundle_dir().unwrap();
        let rootfs = testutil::init_rootfs_dir(&bundle).unwrap();
        testutil::init_spec_file(&bundle, &rootfs).unwrap();
        let spec = specutil::load(&bundle).unwrap();

        let meta_dir = PathBuf::from(DEFAULT_META_ROOT).join(&container_id);

        let mut container = Container::new(&container_id, &bundle, spec);
        assert!(container.create().is_ok());

        let args = vec!["runt", "delete", &container_id];

        let app_matches = app_config()
            .get_matches_from_safe(&args)
            .unwrap_or_else(|e| panic!("An error occurs: {}", e));

        let subcommand = match app_matches.subcommand() {
            ("delete", Some(matches)) => Some(DeleteCommand::new(matches)),
            _ => None,
        }
        .unwrap()
        .unwrap();

        assert!(subcommand.run().is_ok());
        assert!(!meta_dir.exists());

        testutil::cleanup(&[&bundle, &meta_dir]).unwrap();
    }
}
