use clap::{crate_authors, crate_name, crate_version, App, Arg, SubCommand};

pub fn app_config<'a>() -> App<'a, 'a> {
    // Initialize Application
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("It's OCI runtime.");

    // Root options {{
    let root_arg = Arg::with_name("root")
        .required(false)
        .takes_value(true)
        .long("root");

    let debug_flag = Arg::with_name("debug").takes_value(false).long("debug");

    let log_arg = Arg::with_name("log")
        .required(false)
        .takes_value(true)
        .long("log");

    let log_format_arg = Arg::with_name("log-format")
        .required(false)
        .takes_value(true)
        .long("log-format");
    // }}

    // Offen use arguments {{
    let bundle_arg = Arg::with_name("bundle")
        .takes_value(true)
        .long("bundle")
        .short("b");

    let pid_file_arg = Arg::with_name("pid-file")
        .takes_value(true)
        .long("pid-file");

    let console_socket_arg = Arg::with_name("console-socket")
        .takes_value(true)
        .required(false)
        .long("console-socket");

    let container_id_arg = Arg::with_name("container-id")
        .required(true)
        .takes_value(true);
    // }}

    // SubCommands {{
    let create_command = SubCommand::with_name("create")
        .about("create container")
        .arg(&bundle_arg)
        .arg(&pid_file_arg)
        .arg(&console_socket_arg)
        .arg(&container_id_arg);

    let start_command = SubCommand::with_name("start")
        .about("start container")
        .arg(&container_id_arg);

    let run_command = SubCommand::with_name("run")
        .about("run container")
        .arg(&bundle_arg)
        .arg(&pid_file_arg)
        .arg(&console_socket_arg)
        .arg(&container_id_arg);

    let state_command = SubCommand::with_name("state")
        .about("print container state")
        .arg(&container_id_arg);

    let delete_command = SubCommand::with_name("delete")
        .about("delete rootfs, state")
        .arg(
            Arg::with_name("force delete container")
                .required(false)
                .takes_value(false)
                .long("force")
                .short("f"),
        )
        .arg(&container_id_arg);

    let kill_command = SubCommand::with_name("kill")
        .about("kill container")
        .arg(&container_id_arg)
        .arg(Arg::with_name("signal").takes_value(true).required(true));

    let spec_command = SubCommand::with_name("spec")
        .about("generate spec file")
        .arg(&bundle_arg);
    // }}

    app.arg(root_arg)
        .arg(log_arg)
        .arg(debug_flag)
        .arg(log_format_arg)
        .subcommand(create_command)
        .subcommand(start_command)
        .subcommand(run_command)
        .subcommand(state_command)
        .subcommand(delete_command)
        .subcommand(kill_command)
        .subcommand(spec_command)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_command_minimum_args() {
        let args = vec!["runt", "create", "container-a"];

        app_config()
            .get_matches_from_safe(&args)
            .unwrap_or_else(|e| panic!("An error occurs: {}", e));
    }

    #[test]
    fn test_create_command_maximum_args() {
        let args = vec![
            "runt",
            "create",
            "--console-socket",
            "console.sock",
            "--pid-file",
            "container.pid",
            "--bundle",
            "./",
            "container-a",
        ];

        app_config()
            .get_matches_from_safe(&args)
            .unwrap_or_else(|e| panic!("An error occurs: {}", e));
    }

    #[test]
    fn test_start_command() {
        let args = vec!["runt", "start", "container-a"];

        app_config()
            .get_matches_from_safe(&args)
            .unwrap_or_else(|e| panic!("An error occurs: {}", e));
    }

    #[test]
    fn test_run_command_minimum() {
        let args = vec!["runt", "run", "container-a"];

        app_config()
            .get_matches_from_safe(&args)
            .unwrap_or_else(|e| panic!("An error occurs: {}", e));
    }

    #[test]
    fn test_run_command_maximum() {
        let args = vec![
            "runt",
            "run",
            "--bundle",
            "./",
            "--pid-file",
            "container.pid",
            "--console-socket",
            "console.sock",
            "container-a",
        ];

        app_config()
            .get_matches_from_safe(&args)
            .unwrap_or_else(|e| panic!("An error occurs: {}", e));
    }

    #[test]
    fn test_state_command() {
        let args = vec!["runt", "state", "container-a"];

        app_config()
            .get_matches_from_safe(&args)
            .unwrap_or_else(|e| panic!("An error occurs: {}", e));
    }

    #[test]
    fn test_state_command_must_specify_container_id() {
        let args = vec!["runt", "state"];

        app_config()
            .get_matches_from_safe(&args)
            .expect_err("should be error");
    }

    #[test]
    fn test_delete_command() {
        let args = vec!["runt", "delete", "container-a"];

        app_config()
            .get_matches_from_safe(&args)
            .unwrap_or_else(|e| panic!("An error occurs: {}", e));
    }

    #[test]
    fn test_delete_command_must_specify_container_id() {
        let args = vec!["runt", "delete"];

        app_config()
            .get_matches_from_safe(&args)
            .expect_err("should be error");
    }

    #[test]
    fn test_kill_command() {
        let args = vec!["runt", "kill", "container-a", "KILL"];

        app_config()
            .get_matches_from_safe(&args)
            .unwrap_or_else(|e| panic!("An error occurs: {}", e));
    }

    #[test]
    fn test_kill_command_must_specify_container_id() {
        let args = vec!["runt", "kill"];

        app_config()
            .get_matches_from_safe(&args)
            .expect_err("should be error");
    }

    #[test]
    fn test_spec_command() {
        let args = vec!["runt", "spec"];

        app_config()
            .get_matches_from_safe(&args)
            .unwrap_or_else(|e| panic!("An error occurs: {}", e));
    }
}
