use std::io::{self, Read};
use std::{env, fmt::Display};

use crate::stats::Stats;

pub struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Options {
    pub most_bytes: bool,
    pub bytes: bool,
    pub lines: bool,
    pub characters: bool,
    pub words: bool,
}

impl Options {
    pub fn parse_options() -> Result<(Options, Vec<String>), ParseError> {
        let mut args = env::args().skip(1);

        let mut options = Options {
            most_bytes: false,
            bytes: false,
            lines: false,
            characters: false,
            words: false,
        };
        let mut file_names: Vec<String> = vec![];

        while let Some(arg) = args.next() {
            if arg.starts_with('-') {
                if let Some(err) = options.decode_options(arg) {
                    return Err(err);
                }
            } else {
                file_names.push(arg);
            }
        }

        if !options.characters && !options.lines && !options.bytes && !options.words && !options.most_bytes {
            options.lines = true;
            options.words = true;
            options.bytes = true;
        }

        if file_names.len() == 0 {
            let mut content = String::new();
            io::stdin().read_to_string(&mut content).unwrap();
            let stats = Stats::get_stats(content, &options);
            stats.display(&options, &String::from(""));
        }

        Ok((options, file_names))
    }

    fn decode_options(&mut self, arg: String) -> Option<ParseError> {
        let mut chars = arg.chars();
        chars.next();

        while let Some(c) = chars.next() {
            match c {
                'L' => self.most_bytes = true,
                'c' => self.bytes = true,
                'l' => self.lines = true,
                'm' => self.characters = true,
                'w' => self.words = true,
                _ => {
                    return Some(ParseError(format!(
                        "wcw: illegal option -- {}\nusage: wcw [-Lclmw] [file ...]",
                        c
                    )));
                }
            }
        }

        None
    }
}
