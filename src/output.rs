use crate::word::Word;
use colored::*;

pub fn format_output(entry: &str, data: String) {
    if data.len() > 0 {
        println!("\n  {}\n    {}", entry.bold(), data);
    }
}

pub fn output_data(word: Word, args: Option<&str>) {
    let mode = OutputMode::new(word.clone());

    println!("{}", word.word.bold()); 
    match args {
        Some(m) => match m {
            "usage" => mode.usage(),
            "definition" => mode.definition(),
            "etymology" => mode.etymology(),
            "data" => mode.ku_data(),
            "also" => mode.see_also(),
            _ => std::process::exit(1),
        },
        None => {
            mode.usage();
            mode.definition();
            mode.etymology();
            mode.ku_data();
            mode.see_also();
        }
    }
}

pub struct OutputMode {
    word: Word,
}

impl OutputMode {
    pub fn new(word: Word) -> Self {
        OutputMode { word }
    }

    fn usage(&self) {
        format_output("usage", format!(
            "{} ({})", self.word.usage, self.word.book)
        );
    }

    fn definition(&self) {
        format_output("definition", self.word.definition.to_string());
    }

    fn etymology(&self) {
        let etymology: Vec<String> = self.word.etymology.iter()
            .map(|(_, value)| value.to_string())
            .collect();
        format_output("etymology", format!(
            "{}: {}", self.word.source_lang, etymology.join("; ")
        ));
    }

    fn ku_data(&self) {
        let ku_data: Vec<String> = self.word.ku_data.iter()
            .map(|(key, value)| format!("{}: {}%", key, value))
            .collect();
        format_output("ku data", ku_data.join(", "));
    }

    fn see_also(&self) {
        let see_also: Vec<String> = self.word.see_also.iter()
            .filter_map(|value| value.as_str())
            .map(|s| s.to_string())
            .collect();
        format_output("see also", see_also.join(", "));
    }
}
