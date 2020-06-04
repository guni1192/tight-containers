use anyhow::Result;
use clap::ArgMatches;

use crate::container::specs::State;
use crate::container::{Container, MetadataManager};
use crate::subcommand::SubCommandImpl;

pub struct StateCommand {
    pub container_id: String,
}

impl SubCommandImpl for StateCommand {
    fn new(matches: &ArgMatches) -> Result<Self> {
        let container_id = matches
            .value_of("container-id")
            .expect("container-id must be specify:");

        Ok(StateCommand {
            container_id: container_id.into(),
        })
    }
    // TODO: Integrate testing along each scenario
    fn run(&self) -> Result<()> {
        let container = Container::load(&self.container_id)?;
        let state: State = container.state()?;
        println!("{}", serde_json::to_string_pretty(&state)?);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use uuid::Uuid;

    use crate::cli::app_config;

    #[test]
    fn state_command_shuould_be_successfull() {
        let container_id = Uuid::new_v4().to_string();
        let args = vec!["runt", "state", &container_id];

        let app_matches = app_config()
            .get_matches_from_safe(&args)
            .unwrap_or_else(|e| panic!("An error occurs: {}", e));

        let subcommand = match app_matches.subcommand() {
            ("state", Some(matches)) => Some(StateCommand::new(matches)),
            _ => None,
        }
        .unwrap()
        .unwrap();

        assert_eq!(subcommand.container_id, container_id);
    }
}
