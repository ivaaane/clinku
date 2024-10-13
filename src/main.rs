use reqwest::Error;
use serde_json::{json, Map, Value};

#[derive(Debug)]
struct Word<'a> {
    word: &'a str,
    book: &'a str,
    usage: i64,
    definition: &'a str,
    ku_data: &'a Map<String, Value>,
    see_also: &'a Vec<Value>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://api.linku.la/v1/words/soko";
    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    let json = json!(response);

    let word = Word {
        word: json["word"].as_str().unwrap(),
        book: json["book"].as_str().unwrap(),
        usage: json["usage"]["2024-09"].as_i64().unwrap(),
        definition: json["translations"]["en"]["definition"].as_str().unwrap(),
        ku_data: json["ku_data"].as_object().unwrap(),
        see_also: json["see_also"].as_array().unwrap(),
    };

    println!("{}", word.word); 
    println!("{}", word.book);
    println!("{}", word.usage);
    println!("{}", word.definition);
    println!("{:?}", word.ku_data);
    println!("{:?}", word.see_also);

    Ok(())
}
