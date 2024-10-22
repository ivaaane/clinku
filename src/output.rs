use crate::word::Word;
use colored::*;

pub fn format_output(entry: &str, data: String) {
    if data.len() > 0 {
        println!("  {}\n    {}\n", entry.bold(), data);
    }
}

pub fn output_data(word: &Word) {
    println!("{}\n", word.word.bold()); 
    format_output("usage", format!("{} ({})", word.usage, word.book));
    format_output("definition", word.definition.to_string());

    let etymology: Vec<String> = word.etymology.iter()
        .map(|(_, value)| value.to_string())
        .collect();
    format_output("etymology", format!("{}: {}", word.source_lang, etymology.join("; ")));

    let ku_data: Vec<String> = word.ku_data.iter()
        .map(|(key, value)| format!("{}: {}%", key, value))
        .collect();
    format_output("ku data", ku_data.join(", "));

    let see_also: Vec<String> = word.see_also.iter()
        .filter_map(|value| value.as_str())
        .map(|s| s.to_string())
        .collect();
    format_output("see also", see_also.join(", "));

    println!("{}{}", "https://linku.la/words/".bright_black(), word.word.bright_black());
}
