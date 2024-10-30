use crate::word::Word;
use colored::*;

pub fn format_output(entry: &str, data: String, header: bool) {
    if !data.is_empty() {
        if header {
            print!("{}: ", entry.bold().underline());
        }
        print!("{}\n", data);
    }
}

pub fn output_data(word: Word, args: Option<&str>) {
    let mode = OutputMode::new(word.clone());

    match args {
        Some(m) => match m {
            "usage" => mode.usage(false),
            "definition" => mode.definition(false),
            "etymology" => mode.etymology(false),
            "data" => mode.ku_data(false),
            "also" => mode.see_also(false),
            _ => std::process::exit(1),
        },
        None => {
            mode.usage(true);
            mode.definition(true);
            mode.etymology(true);
            mode.ku_data(true);
            mode.see_also(true);
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

    fn usage(&self, header: bool) {
        format_output("usage", format!(
            "{} ({})", self.word.usage, self.word.book),
        header);
    }

    fn definition(&self, header: bool) {
        format_output("definition", self.word.definition.to_string(), header);
    }

    fn etymology(&self, header: bool) {
        let etymology: Vec<String> = self.word.etymology.iter()
            .map(|(_, value)| value.to_string())
            .collect();
        format_output("etymology", format!(
            "{} {}", self.word.source_lang, etymology.join("; ")
        ), header);
    }

    fn ku_data(&self, header: bool) {
        let ku_data: Vec<String> = self.word.ku_data.iter()
            .map(|(key, value)| {
                let percent = match value.as_i64().unwrap_or(0) {
                    0..=20 => "²",
                    21..=50 => "³",
                    51..=75 => "⁴",
                    76..=100 => "⁵",
                    _ => "?",
                };
                format!("{}{}", key, percent)
            })
            .collect();
        format_output("ku data", ku_data.join(", "), header);
    }

    fn see_also(&self, header: bool) {
        let see_also: Vec<String> = self.word.see_also.iter()
            .filter_map(|value| value.as_str())
            .map(|s| s.to_string())
            .collect();
        format_output("see also", see_also.join(", "), header);
    }
}
