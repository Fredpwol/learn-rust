use mini_grep::{get_config_data, search_text};
use std::{process, env};
fn main() {
    let variables: Vec<String> = env::args().collect();
    let config = get_config_data(&variables).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    search_text(config).unwrap_or_else(|err|{
        eprintln!("{}", err);
    });
}
