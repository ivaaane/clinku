use serde_json::{Map, Value};

pub struct Word<'a> {
    pub word: &'a str,
    pub book: &'a str,
    pub usage: &'a str,
    pub definition: &'a str,
    pub ku_data: &'a Map<String, Value>,
    pub see_also: &'a Vec<Value>,
    pub source_lang: &'a str,
    pub etymology: &'a Map<String, Value>,
}

impl<'a> Word<'a> {
    pub fn new(json: &'a serde_json::Value) -> Self {
        Word {
            word: json["word"].as_str().unwrap(),
            book: json["book"].as_str().unwrap(),
            usage: json["usage_category"].as_str().unwrap(),
            definition: json["translations"]["en"]["definition"].as_str().unwrap(),
            ku_data: json["ku_data"].as_object().unwrap(),
            see_also: json["see_also"].as_array().unwrap(),
            source_lang: json["source_language"].as_str().unwrap(),
            etymology: json["etymology"].as_array().unwrap()[0].as_object().unwrap(),
        }
    }
}
