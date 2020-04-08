mod args;
mod container;
mod error;
mod subcommand;

fn main() {
    let app_maches = args::app_config().get_matches();

    match app_maches.subcommand() {
        ("spec", Some(matches)) => subcommand::spec::run(&matches),
        _ => eprintln!("Unexpect argument"),
    }
}
