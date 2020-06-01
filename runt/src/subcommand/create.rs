use std::path::PathBuf;

use anyhow::Result;
use clap::ArgMatches;

use crate::container::Container;
use crate::specutil;
use crate::subcommand::SubCommandImpl;

pub struct CreateCommand {
    pub container_id: String,
    pub bundle: PathBuf,
    pub pid_file: Option<PathBuf>,
    pub console_socket: Option<PathBuf>,
}

impl SubCommandImpl for CreateCommand {
    fn new(matches: &ArgMatches) -> Result<Self> {
        let container_id = matches.value_of("container-id").unwrap();
        let bundle = PathBuf::from(matches.value_of("bundle").unwrap_or("."));
        let pid_file = matches.value_of("pid-file").map(PathBuf::from);
        let console_socket = matches.value_of("console-socket").map(PathBuf::from);
        Ok(CreateCommand {
            container_id: container_id.into(),
            bundle,
            pid_file,
            console_socket,
        })
    }

    fn run(&self) -> Result<()> {
        let spec = specutil::load(&self.bundle)?;
        let container = Container::new(&self.container_id, &self.bundle, spec);
        container.create()?;

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use uuid::Uuid;

    use crate::cli::app_config;
    use crate::container::testutil;

    fn init_create_command(args: Vec<&str>) -> CreateCommand {
        let app_matches = app_config()
            .get_matches_from_safe(&args)
            .expect("arg matche err: ");

        match app_matches.subcommand() {
            ("create", Some(matches)) => Some(CreateCommand::new(matches)),
            _ => None,
        }
        .unwrap()
        .unwrap()
    }

    #[test]
    #[should_panic]
    fn if_container_id_is_none_expect_err() {
        let args = vec!["runt", "create"];
        let _create_command = init_create_command(args);
    }

    #[test]
    fn bundle_should_be_current_dir() {
        let container_id = Uuid::new_v4().to_string();
        let args = vec!["runt", "create", &container_id];

        let create_command = init_create_command(args);

        assert_eq!(create_command.container_id, container_id);
        assert_eq!(create_command.bundle, PathBuf::from("."));
        assert_eq!(create_command.console_socket, None);
        assert_eq!(create_command.pid_file, None);
    }

    #[test]
    fn bundle_should_be_specify_dir() {
        let container_id = Uuid::new_v4().to_string();
        let bundle = testutil::init_bundle_dir(&container_id).unwrap();

        let args = vec![
            "runt",
            "create",
            "-b",
            bundle.to_str().unwrap(),
            &container_id,
        ];

        let create_command = init_create_command(args);

        assert_eq!(create_command.container_id, container_id);
        assert_eq!(create_command.bundle, bundle);
        assert_eq!(create_command.console_socket, None);
        assert_eq!(create_command.pid_file, None);
        testutil::cleanup(&bundle).unwrap();
    }

    #[test]
    fn console_socket_should_be_specify_path() {
        let container_id = Uuid::new_v4().to_string();
        let bundle = testutil::init_bundle_dir(&container_id).unwrap();
        let args = vec![
            "runt",
            "create",
            "-b",
            bundle.to_str().unwrap(),
            "--console-socket",
            "/tmp/console.sock",
            &container_id,
        ];

        let create_command = init_create_command(args);

        assert_eq!(create_command.container_id, container_id);
        assert_eq!(create_command.bundle, bundle);
        assert_eq!(
            create_command.console_socket,
            Some(PathBuf::from("/tmp/console.sock"))
        );
        assert_eq!(create_command.pid_file, None);
        testutil::cleanup(&bundle).unwrap();
    }

    #[test]
    fn pid_file_should_be_specify_path() {
        let container_id = Uuid::new_v4().to_string();
        let bundle = testutil::init_bundle_dir(&container_id).unwrap();
        let args = vec![
            "runt",
            "create",
            "--bundle",
            bundle.to_str().unwrap(),
            "--console-socket",
            "/tmp/console.sock",
            "--pid-file",
            "/tmp/container.pid",
            &container_id,
        ];

        let create_command = init_create_command(args);

        assert_eq!(create_command.container_id, container_id);
        assert_eq!(create_command.bundle, bundle);
        assert_eq!(
            create_command.console_socket,
            Some(PathBuf::from("/tmp/console.sock"))
        );
        assert_eq!(
            create_command.pid_file,
            Some(PathBuf::from("/tmp/container.pid"))
        );
        testutil::cleanup(&bundle).unwrap();
    }

    #[test]
    fn create_shuould_be_success() {
        let container_id = Uuid::new_v4().to_string();
        let bundle = testutil::init_bundle_dir(&container_id).unwrap();
        testutil::init_spec_file(&bundle).unwrap();

        let args = vec![
            "runt",
            "create",
            "--bundle",
            bundle.to_str().unwrap(),
            &container_id,
        ];

        let create_command = init_create_command(args);

        assert!(create_command.run().is_ok());

        testutil::cleanup(&bundle).unwrap();
    }
}
