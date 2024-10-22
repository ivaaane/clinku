pub mod request;
pub mod word;
pub mod output;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        print_help();
    }

    let json = request::fetch_api(&args[1]).await?;
    let word = word::Word::new(&json);
    output::output_data(&word);

    Ok(())
}

fn print_help() {
    println!("TO DO!!!");
}
