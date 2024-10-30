pub mod request;
pub mod word;
pub mod output;

use reqwest::Error;
use clap::{arg, Command};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = cli().get_matches();
    let word = matches.get_one::<String>("WORD").expect("required");
    let sandbox = matches.get_flag("sandbox");

    let mode = if matches.get_flag("usage") {
        Some("usage")
    } else if matches.get_flag("definition") {
        Some("definition")
    } else if matches.get_flag("etymology") {
        Some("etymology")
    } else if matches.get_flag("data") {
        Some("data")
    } else if matches.get_flag("also") {
        Some("also")
    } else {
        None
    };

    let json = request::fetch_api(word, sandbox).await?;
    let word = word::Word::new(&json);
    output::output_data(word, mode);

    Ok(())
}

fn cli() -> Command {
    Command::new("clinku")
        .about("A CLI to interact with the Linku database")
        .arg_required_else_help(true)
        .arg(arg!(<WORD> "The word to search up"))
        .arg(arg!(--sandbox "Search for words in the sandbox"))
        .arg(arg!(--usage "Only show the usage"))
        .arg(arg!(--definition "Only show the definition"))
        .arg(arg!(--etymology "Only show the etymology"))
        .arg(arg!(--data "Only show the ku data"))
        .arg(arg!(--also "Only show the 'see also'"))
}
