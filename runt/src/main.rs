mod args;

fn main() {
    let app_maches = args::app_config().get_matches();

    match app_maches.subcommand() {
        ("spec", _matches) => {}
        _ => eprintln!("Unexpect argument"),
    }
}
