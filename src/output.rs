use crate::word::Word;

pub fn output_data(word: &Word) {
    println!("=> word: {}", word.word); 
    println!("=> usage: {} ({})", word.usage, word.book);
    println!("=> definition: {}", word.definition);

    let etymology: Vec<String> = word.etymology.iter()
        .map(|(_, value)| value.to_string())
        .collect();
    println!("-> etymology: {}: {}", word.source_lang, etymology.join("; "));

    let ku_data: Vec<String> = word.ku_data.iter()
        .map(|(key, value)| format!("{}: {}%", key, value))
        .collect();
    println!("=> ku data: {}", ku_data.join(", "));

    let see_also: Vec<String> = word.see_also.iter()
        .filter_map(|value| value.as_str())
        .map(|s| s.to_string())
        .collect();
    println!("=> see also: {}", see_also.join(", "));
}
