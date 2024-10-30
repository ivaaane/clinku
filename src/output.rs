use crate::word::Word;
use colored::*;

// format output to make it *pretty*
pub fn format_output(entry: &str, data: String, header: bool) {
    if !data.is_empty() { // don't print anything if the field is empty
        // if only one field is outputted, then don't print the
        // header. if all fields were present, we do need it to
        // differenciate.
        if header {
            print!("{}: ", entry.bold().underline());
        }
        print!("{}\n", data); // the data however is always present
    }
}

// main function to be called
pub fn output_data(word: Word, args: Option<&str>) {
    // make new instance of the OutputMode struct
    let mode = OutputMode::new(word.clone());

    // check if we should print all fields or only one based on args
    match args {
        // in case there are arguments, check which one
        Some(m) => match m {
            // the booleans indicates if the header should be printed
            "usage" => mode.usage(false),
            "definition" => mode.definition(false),
            "etymology" => mode.etymology(false),
            "data" => mode.ku_data(false),
            "also" => mode.see_also(false),
            _ => std::process::exit(1), // this shouldn't happen
        },
        // if no arguments, print everything
        None => {
            mode.usage(true);
            mode.definition(true);
            mode.etymology(true);
            mode.ku_data(true);
            mode.see_also(true);
        }
    }
}

// this struct contains methods telling how to output each field of data
pub struct OutputMode {
    word: Word,
}

impl OutputMode {
    // create new instance
    pub fn new(word: Word) -> Self {
        OutputMode { word }
    }

    // the following are the fields

    fn usage(&self, header: bool) {
        format_output("usage", format!(
            "{} ({})", self.word.usage, self.word.book),
        header);
    }

    fn definition(&self, header: bool) {
        format_output("definition", self.word.definition.to_string(), header);
    }

    // fields consisting of vectors or hashmaps need special treatment to extract
    // the data
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
                // this makes the pretty range numbers as seen in lipu ku
                let percent = match value.as_i64().unwrap_or(0) {
                    11..=20  => "¹",
                    21..=40  => "²",
                    41..=60  => "³",
                    61..=80  => "⁴",
                    81..=100 => "⁵",
                    _ => "½",
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
