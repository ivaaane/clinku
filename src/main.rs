use std::env;
use reqwest::Error;
use serde_json::{json, Map, Value};

struct Word<'a> {
    word: &'a str,
    book: &'a str,
    usage: &'a str,
    definition: &'a str,
    ku_data: &'a Map<String, Value>,
    see_also: &'a Vec<Value>,
//  source_lang: &'a str,
//  etymology: &'a Vec<Value>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("One argument is needed");
    }

    let url = format!("https://api.linku.la/v1/words/{}", args[1]);
    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    let json = json!(response);

    let word = Word {
        word: json["word"].as_str().unwrap(),
        book: json["book"].as_str().unwrap(),
        usage: json["usage_category"].as_str().unwrap(),
        definition: json["translations"]["en"]["definition"].as_str().unwrap(),
        ku_data: json["ku_data"].as_object().unwrap(),
        see_also: json["see_also"].as_array().unwrap(),
//      source_lang: json["source_language"].as_str().unwrap(),
//      etymology: json["etymology"].as_array().unwrap(),
    };

    println!("=> word: {}", word.word); 
    println!("=> usage: {} ({})", word.usage, word.book);
    println!("=> definition: {}", word.definition);
/*
    let etymology = word.etymology.get(0).and_then(Value::as_object);
    let etymology: Vec<String> = etymology.iter()
        .filter_map(|value| value.as_str()
        .map(|s| s.to_string()))
        .collect();
    println!("-> etymology: {}: {}", word.source_lang, etymology.join("; "));
*/
    let ku_data: Vec<String> = word.ku_data.iter()
        .map(|(key, value)| format!("{}: {}%", key, value))
        .collect();
    println!("=> ku data: {}", ku_data.join(", "));

    let see_also: Vec<String> = word.see_also.iter()
        .filter_map(|value| value.as_str())
        .map(|s| s.to_string())
        .collect();
    println!("=> see also: {}", see_also.join(", "));

    Ok(())
}
