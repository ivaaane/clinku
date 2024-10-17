use crate::word::Word;
use colored::*;

pub fn output_data(word: &Word) {
    println!("{}\n", word.word.bold()); 
    println!("{}\n    {} ({})\n", "usage".bold(), word.usage, word.book);
    println!("{}\n    {}\n", "definition".bold(), word.definition);

    let etymology: Vec<String> = word.etymology.iter()
        .map(|(_, value)| value.to_string())
        .collect();
    println!("{}\n    {}: {}\n", "etymology".bold(), word.source_lang, etymology.join("; "));

    let ku_data: Vec<String> = word.ku_data.iter()
        .map(|(key, value)| format!("{}: {}%", key, value))
        .collect();
    println!("{}\n    {}\n", "ku data".bold(), ku_data.join(", "));

    let see_also: Vec<String> = word.see_also.iter()
        .filter_map(|value| value.as_str())
        .map(|s| s.to_string())
        .collect();
    println!("{}\n    {}", "see also".bold(), see_also.join(", "));
}
