use crate::word::Word;
use colored::*;
use textwrap::{wrap, Options};

// format output to make it *pretty*
pub fn format_output(entry: &str, data: String, header: bool) {
     // don't print anything if the field is empty
    if data.is_empty() { return }

    // get the size of the terminal
    // this is for the text wrapping
    let width: usize;
    let size = termsize::get();
    match size {
        Some(dimension) => width = dimension.cols.try_into().unwrap(),
        None => width = 1,
    }
    
    // use textwrap to wrap text and make it *pretty*

    // a differenciation is made between the first line and the
    // other lines, because the first line needs its limit to
    // be the double of the normal one (reasons).
    let first_line = &wrap(
        &data,
        Options::new(width - (entry.len() * 2))
    )[0];

    let next_lines = wrap(
        &data,
        Options::new(width - entry.len())
            .subsequent_indent(&" ".repeat(entry.len() + 2))
    )[1..].join("\n");
    
    // a newline is needed in the end if more than one line exists
    let next_lines = if next_lines.is_empty() {
        next_lines
    } else {
        next_lines + "\n" 
    }; // honestly i hate this

    // both parts are then joined and like nothing happened
    let data_wrap = format!("{}\n{}", first_line, next_lines);

    if header {
        // if only one field is outputted, then don't print the
        // header. if all fields were present, we do need it
        println!("{}: {}", entry.bold().underline(), data_wrap);
    } else {
        // in case the header isn't pressent, use plain
        // text instead of making it look good
        println!("{}", data);
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
        format_output("Usage", format!(
            "{} ({})", self.word.usage, self.word.book),
        header);
    }

    fn definition(&self, header: bool) {
        format_output("Definition", self.word.definition.to_string(), header);
    }

    // fields consisting of vectors or hashmaps need special treatment to extract
    // the data
    fn etymology(&self, header: bool) {
        let etymology: Vec<String> = self.word.etymology.iter()
            .map(|(_, value)| value.to_string())
            .collect();
        format_output("Etymology", format!(
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
        format_output("See also", see_also.join(", "), header);
    }
}
