use serde_json::{Map, Value};

// this has all the wanted fields of a word in Linku
#[derive(Clone)]
pub struct Word {
    pub word: String,
    pub book: String,
    pub usage: String,
    pub definition: String,
    pub ku_data: Map<String, Value>,
    pub see_also: Vec<Value>,
    pub source_lang: String,
    pub etymology: Map<String, Value>,
    pub commentary: String,
    pub creator: Vec<Value>,
}

impl Word {
    pub fn new(json: &Value) -> Self {
        // extract json data to common data types
        Word {
            word: json["word"]
                // if the field doesn't exist, leave it blank.
                // &str is converted to String
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
                // in etymology, we only get the first item of the vector
                .as_array().unwrap_or(&Vec::new()).clone()[0]
                .as_object().unwrap_or(&Map::new()).clone(),
            commentary: json["translations"]["en"]["commentary"]
                .as_str().unwrap_or("").to_string(),
            creator: json["creator"]
                .as_array().unwrap_or(&Vec::new()).clone(),
        }
    }
}
