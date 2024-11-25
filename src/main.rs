pub mod request;
pub mod word;
pub mod output;

use reqwest::Error;
use clap::{arg, Command};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // parse arguments
    let matches = cli().get_matches();
    let word = matches.get_one::<String>("WORD").expect("required");
    let sandbox = matches.get_flag("sandbox");

    // check flags to determine which data to output
    let mode = if matches.get_flag("usage") {
        Some("usage")
    } else if matches.get_flag("definition") {
        Some("definition")
    } else if matches.get_flag("etymology") {
        Some("etymology")
    } else if matches.get_flag("ku") {
        Some("data")
    } else if matches.get_flag("see_also") {
        Some("also")
    } else if matches.get_flag("commentary") {
        Some("commentary")
    } else if matches.get_flag("creator") {
        Some("creator")
    } else {
        None // if no flag, it'll output all data
    };

    // fetch data
    let json = request::fetch_api(word, sandbox).await?;
    let word = word::Word::new(&json);

    output::output_data(word, mode); // output the data

    Ok(())
}

// use clap for the CLI
fn cli() -> Command {
    Command::new("clinku")
        .about("A CLI to interact with the Linku dictionary")
        .arg_required_else_help(true)
        .arg(arg!(<WORD> "The word to search up")) // mandatory arg

        // optional flags
        .arg(arg!(-s --sandbox "Search for words in the sandbox"))
        .arg(arg!(-u --usage "Only show the usage"))
        .arg(arg!(-d --definition "Only show the definition"))
        .arg(arg!(-e --etymology "Only show the etymology"))
        .arg(arg!(-k --ku "Only show the ku data"))
        .arg(arg!(-a --see_also "Only show the 'see also'"))
        .arg(arg!(-c --commentary "Only show the commentary"))
        .arg(arg!(-b --creator "Only show the creator/s"))
}
