use serde_json::{Map, Value};

pub struct Word {
    pub word: String,
    pub book: String,
    pub usage: String,
    pub definition: String,
    pub ku_data: Map<String, Value>,
    pub see_also: Vec<Value>,
    pub source_lang: String,
    pub etymology: Map<String, Value>,
}

impl Word {
    pub fn new(json: &Value) -> Self {
        Word {
            word: json["word"]
                .as_str().unwrap_or("").to_string(),
            book: json["book"]
                .as_str().unwrap_or("").to_string(),
            usage: json["usage_category"]
                .as_str().unwrap_or("").to_string(),
            definition: json["translations"]["en"]["definition"]
                .as_str().unwrap_or("").to_string(),
            ku_data: json["ku_data"]
                .as_object().unwrap_or(&Map::new()).clone(),
            see_also: json["see_also"]
                .as_array().unwrap_or(&Vec::new()).clone(),
            source_lang: json["source_language"]
                .as_str().unwrap_or("").to_string(),
            etymology: json["etymology"]
                .as_array().unwrap_or(&Vec::new()).clone()[0]
                .as_object().unwrap_or(&Map::new()).clone(),
        }
    }
}
